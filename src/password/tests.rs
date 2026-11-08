use super::*;
use std::time::{Duration, Instant};

const VECTOR: &str =
    "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4";

#[test]
fn verifies_upstream_known_answer() {
    assert_eq!(verify(b"password", VECTOR), Ok(true));
    assert_eq!(verify(b"different", VECTOR), Ok(false));
}

#[test]
fn random_salts_and_default_phc_parameters() {
    let first = hash(b"test password").unwrap();
    let second = hash(b"test password").unwrap();
    assert_ne!(first, second);
    assert!(first.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
    assert!(first.len() < 128);
    assert_eq!(verify(b"test password", &first), Ok(true));
    assert_eq!(verify(b"wrong", &first), Ok(false));
}

#[test]
fn supports_empty_and_original_non_utf8_bytes() {
    let empty = hash(b"").unwrap();
    assert_eq!(verify(b"", &empty), Ok(true));
    let raw = hash(&[0xe9]).unwrap();
    assert_eq!(verify(&[0xe9], &raw), Ok(true));
    assert_eq!(verify("\u{e9}".as_bytes(), &raw), Ok(false));
    assert_eq!(verify("\u{fffd}".as_bytes(), &raw), Ok(false));
}

#[test]
fn refuses_bad_phc_and_excessive_cost_before_allocation() {
    for stored in [
        "not-a-hash".to_string(),
        "$argon2id$v=19$m=256,t=2,p=1".to_string(),
        VECTOR.replace("argon2id", "argon2i"),
        VECTOR.replace("v=19", "v=16"),
        VECTOR.replace("m=256", "m=65537"),
        VECTOR.replace("t=2", "t=11"),
        VECTOR.replace("p=1", "p=5"),
        VECTOR.replace("m=256", "m=4294967295"),
        "x".repeat(MAX_HASH_LEN + 1),
    ] {
        assert_eq!(verify(b"password", &stored), Err(Status::InvalidHash));
    }
}

#[test]
fn permits_bound_queued_running_and_posted_work() {
    let pending = Arc::new(AtomicUsize::new(0));
    let mut permits: Vec<_> = (0..MAX_PENDING)
        .map(|_| Permit::acquire(&pending).unwrap())
        .collect();
    assert!(Permit::acquire(&pending).is_none());
    permits.pop();
    assert!(Permit::acquire(&pending).is_some());
    drop(permits);
    assert_eq!(pending.load(Ordering::Acquire), 0);
}

#[test]
fn sdk_delivery_is_cancelled_and_workers_join_on_shutdown() {
    let mut manager = PasswordManager::new();
    let alive = Arc::new(AtomicBool::new(true));
    let callback = Callback {
        script: AmxIdent::from(std::ptr::null_mut()),
        name: "Unused".to_string(),
        context: 7,
        alive: Arc::clone(&alive),
    };
    assert!(manager.submit(
        Zeroizing::new(b"password".to_vec()),
        Some("invalid".to_string()),
        callback
    ));
    let deadline = Instant::now() + Duration::from_secs(5);
    while samp::mainthread::pending() == 0 && Instant::now() < deadline {
        thread::yield_now();
    }
    assert!(samp::mainthread::pending() > 0);
    // The pending permit stays reserved until the SDK processes the result.
    assert_eq!(manager.pending.load(Ordering::Acquire), 1);
    alive.store(false, Ordering::Release);
    manager.shutdown();
    while samp::mainthread::pending() != 0 {
        samp::mainthread::run_pending();
    }
    assert_eq!(manager.pending.load(Ordering::Acquire), 0);
    assert!(manager.workers.is_empty());
    assert!(manager.sender.is_none());
}
