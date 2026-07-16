use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

/*
    Atomic Type: i32, u32, bool
    Atomic Methods: load, store, fecth_and_execute, compare_and_swap
    Memory Ordering: Relaces, Aquired, Released, SeqLock
*/

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100  items

        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // assuming tis takes time
                num_done.store(i + 1, Relaxed);
            }
        });

        //the main thread shows the status completed updates every sec

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working .. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }

    });

    println!("Done!");

}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(37));
}
