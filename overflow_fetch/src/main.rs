use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;


//this version is problematic
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);

    NEXT_ID.fetch_add(1, Relaxed)

}


fn main() {
    dbg!(allocate_new_id()); 
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());

    println!("overflowing the counter..(this might take a minute)");

    for _ in 3..=u32::MAX {
        allocate_new_id();
    }

    println!("overflowed");

    dbg!(allocate_new_id()); //this will produce zero again
}
