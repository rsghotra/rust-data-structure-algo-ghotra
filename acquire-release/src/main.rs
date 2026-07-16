use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // Everything before this Release-store must be visible to others now.
    });

    while !READY.load(Acquire) { //one this Acquire-Load sets to true, i should be seeing all updates beofee the release store
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}
