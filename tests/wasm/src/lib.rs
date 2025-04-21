use std::sync::atomic::{AtomicUsize, Ordering};

static STATE: AtomicUsize = AtomicUsize::new(0);

#[ctor::ctor]
pub fn init() {
    STATE.fetch_add(1, Ordering::Relaxed);
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    assert_eq!(STATE.load(Ordering::Relaxed), 1);
}
