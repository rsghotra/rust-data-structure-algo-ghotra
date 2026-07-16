use std::sync::atomic::{AtomicI32, AtomicU64};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;


fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);

    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    thread::sleep(Duration::from_secs(1));
    123
}



fn main() {
    dbg!(get_x());
    dbg!(get_x());


    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Relaxed);
    let c = a.load(Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}
