//! Bounded Argon2id workers; AMX access is exclusively through rust-samp.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

use argon2::password_hash::{Error, PasswordHasher, PasswordVerifier, phc::PasswordHash};
use argon2::{Algorithm, Argon2, Params, Version};
use samp::amx::AmxIdent;
use samp::exec_public;
use zeroize::Zeroizing;

pub const MAX_PASSWORD_LEN: usize = 1024;
pub const MAX_HASH_LEN: usize = 256;
const MAX_PENDING: usize = 128;
const MAX_WORKERS: usize = 2;
const MAX_MEMORY_KIB: u32 = 65_536;
const MAX_ITERATIONS: u32 = 10;
const MAX_LANES: u32 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    InvalidHash = 1,
    Failed = 2,
}

impl Status {
    fn code(self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::InvalidHash => 1,
            Self::Failed => 2,
        }
    }
}

pub enum Outcome {
    Hash(Result<String, Status>),
    Verify(Result<bool, Status>),
}

pub struct Callback {
    pub script: AmxIdent,
    pub name: String,
    pub context: i32,
    // AmxIdent is an address; a new AMX may reuse it after a restart.
    pub alive: Arc<AtomicBool>,
}

impl Callback {
    fn deliver(self, outcome: Outcome, permit: Permit) {
        samp::mainthread::post(move || {
            let _permit = permit;
            if !self.alive.load(Ordering::Acquire) {
                return;
            }
            let Some(amx) = samp::amx::get(self.script) else {
                return;
            };
            // No plugin borrow is alive here, so callbacks may enqueue new work.
            let result = match outcome {
                Outcome::Hash(result) => {
                    let (status, hash) = match result {
                        Ok(hash) => (Status::Ok, hash),
                        Err(status) => (status, String::new()),
                    };
                    exec_public!(amx, &self.name, status.code(), &hash => string, self.context)
                }
                Outcome::Verify(result) => {
                    let (status, matches) = match result {
                        Ok(matches) => (Status::Ok, matches),
                        Err(status) => (status, false),
                    };
                    exec_public!(amx, &self.name, status.code(), matches, self.context)
                }
            };
            if result.is_err() {
                samp::log::error!("Password callback execution failed.");
            }
        });
    }
}

pub fn hash(password: &[u8]) -> Result<String, Status> {
    Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default())
        .hash_password(password)
        .map(|hash: PasswordHash| hash.to_string())
        .map_err(|_| Status::Failed)
}

pub fn verify(password: &[u8], stored: &str) -> Result<bool, Status> {
    if stored.len() > MAX_HASH_LEN {
        return Err(Status::InvalidHash);
    }
    let parsed = PasswordHash::new(stored).map_err(|_| Status::InvalidHash)?;
    if parsed.algorithm.as_str() != "argon2id"
        || parsed.version != Some(19)
        || parsed.salt.is_none()
        || parsed.hash.is_none()
    {
        return Err(Status::InvalidHash);
    }
    let params = Params::try_from(&parsed).map_err(|_| Status::InvalidHash)?;
    if params.m_cost() > MAX_MEMORY_KIB
        || params.t_cost() > MAX_ITERATIONS
        || params.p_cost() > MAX_LANES
    {
        return Err(Status::InvalidHash);
    }
    match Argon2::default().verify_password(password, &parsed) {
        Ok(()) => Ok(true),
        Err(Error::PasswordInvalid) => Ok(false),
        Err(_) => Err(Status::InvalidHash),
    }
}

/// Counts queued, running AND posted results until delivery or cancellation.
struct Permit(Arc<AtomicUsize>);

impl Permit {
    fn acquire(pending: &Arc<AtomicUsize>) -> Option<Self> {
        pending
            .try_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (count < MAX_PENDING).then_some(count + 1)
            })
            .ok()?;
        Some(Self(Arc::clone(pending)))
    }
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

struct Job {
    password: Zeroizing<Vec<u8>>,
    stored: Option<String>,
    callback: Callback,
    permit: Permit,
}

pub struct PasswordManager {
    sender: Option<mpsc::SyncSender<Job>>,
    workers: Vec<JoinHandle<()>>,
    stopping: Arc<AtomicBool>,
    pending: Arc<AtomicUsize>,
}

impl PasswordManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel::<Job>(MAX_PENDING);
        let receiver = Arc::new(Mutex::new(receiver));
        let stopping = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::new();
        let count = thread::available_parallelism()
            .map_or(1, usize::from)
            .min(MAX_WORKERS);
        for index in 0..count {
            let receiver = Arc::clone(&receiver);
            let stopping = Arc::clone(&stopping);
            match thread::Builder::new()
                .name(format!("crypto-argon2-{index}"))
                .stack_size(512 * 1024)
                .spawn(move || {
                    loop {
                        let job = receiver
                            .lock()
                            .unwrap_or_else(|error| error.into_inner())
                            .recv();
                        let Ok(job) = job else {
                            break;
                        };
                        if stopping.load(Ordering::Acquire)
                            || !job.callback.alive.load(Ordering::Acquire)
                        {
                            continue;
                        }
                        let verifying = job.stored.is_some();
                        let outcome = catch_unwind(AssertUnwindSafe(|| match job.stored {
                            Some(stored) => Outcome::Verify(verify(&job.password, &stored)),
                            None => Outcome::Hash(hash(&job.password)),
                        }))
                        .unwrap_or({
                            if verifying {
                                Outcome::Verify(Err(Status::Failed))
                            } else {
                                Outcome::Hash(Err(Status::Failed))
                            }
                        });
                        if !stopping.load(Ordering::Acquire) {
                            job.callback.deliver(outcome, job.permit);
                        }
                    }
                }) {
                Ok(worker) => workers.push(worker),
                Err(_) => samp::log::error!("Failed to start an Argon2id worker."),
            }
        }
        Self {
            sender: (!workers.is_empty()).then_some(sender),
            workers,
            stopping,
            pending: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn submit(
        &self,
        password: Zeroizing<Vec<u8>>,
        stored: Option<String>,
        callback: Callback,
    ) -> bool {
        if password.len() > MAX_PASSWORD_LEN
            || stored
                .as_ref()
                .is_some_and(|hash| hash.len() > MAX_HASH_LEN)
            || self.stopping.load(Ordering::Acquire)
        {
            return false;
        }
        let Some(sender) = &self.sender else {
            return false;
        };
        let Some(permit) = Permit::acquire(&self.pending) else {
            return false;
        };
        sender
            .try_send(Job {
                password,
                stored,
                callback,
                permit,
            })
            .is_ok()
    }

    pub fn shutdown(&mut self) {
        self.stopping.store(true, Ordering::Release);
        self.sender.take();
        for worker in self.workers.drain(..) {
            if worker.join().is_err() {
                samp::log::error!("An Argon2id worker stopped unexpectedly.");
            }
        }
    }
}

impl Drop for PasswordManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests;
