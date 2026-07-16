use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;


fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);

    let id = NEXT_ID.fetch_add(1, Relaxed);

    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        println!("too many IDs!");
    }

    id
}


fn main() {
    println!("Hello, world!");
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
}
