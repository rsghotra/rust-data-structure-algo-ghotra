use std::{sync::atomic::{AtomicBool, Ordering::{Acquire, Relaxed, Release}}, thread};

static mut DATA: String = String::new();

static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        //safety: we hold the exclusive lock so nothing else is accessing DATA
        unsafe { DATA.push('!')};
        LOCKED.store(true, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
