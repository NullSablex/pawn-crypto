use std::collections::HashMap;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    logger::Logger,
    password::{Callback, PasswordManager},
};
use samp::amx::{Amx, AmxExt, AmxIdent};
use samp::plugin::SampPlugin;

pub struct CryptoPlugin {
    pub passwords: PasswordManager,
    scripts: HashMap<AmxIdent, Arc<AtomicBool>>,
}

impl CryptoPlugin {
    pub fn new() -> Self {
        Logger::init();
        Self {
            passwords: PasswordManager::new(),
            scripts: HashMap::new(),
        }
    }

    pub fn password_callback(&self, amx: &Amx, name: String, context: i32) -> Option<Callback> {
        if name.is_empty() || name.len() > 128 || amx.find_public(&name).is_err() {
            return None;
        }
        Some(Callback {
            script: amx.ident(),
            name,
            context,
            alive: Arc::clone(self.scripts.get(&amx.ident())?),
        })
    }

    fn shutdown(&mut self) {
        for token in self.scripts.values() {
            token.store(false, Ordering::Release);
        }
        self.scripts.clear();
        self.passwords.shutdown();
        // No worker can post now. Cancelled SDK jobs return before touching AMX
        // or the plugin, and must be dropped before unloading the library.
        while samp::mainthread::pending() != 0 {
            samp::mainthread::run_pending();
        }
    }
}

impl SampPlugin for CryptoPlugin {
    fn on_unload(&mut self) {
        self.shutdown();
        samp::log::info!("Plugin unloaded.");
        Logger::flush();
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        if let Some(old) = self
            .scripts
            .insert(amx.ident(), Arc::new(AtomicBool::new(true)))
        {
            old.store(false, Ordering::Release);
        }
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        if let Some(token) = self.scripts.remove(&amx.ident()) {
            token.store(false, Ordering::Release);
        }
    }
}

impl Drop for CryptoPlugin {
    fn drop(&mut self) {
        self.shutdown();
    }
}
