use std::sync::atomic::{AtomicBool, Ordering::Acquire, Ordering::Release};

/*
- we may waste CPU cycles but ultimately it may be a better choice
- Attempting to lock and already locked mutex will result in busy-looping or spinning
- in Busy Loop/Spinning = we keep doing try_lock until we do get the lock acquired


*/

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false)
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) { //lock aquiring
            std::hint::spin_loop(); //see carefully when swap will return true as previous value, it will keep spinning
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release); //lock releasing
    }
}


fn main() {
    println!("Hello, world!");
}
