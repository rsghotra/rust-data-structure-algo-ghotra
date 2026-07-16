use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

fn main() {
    //Synchronization

    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        //A background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i+1, Relaxed);
                main_thread.unpark(); //wake up the main thread
            }
        });

        //the main thread shows update
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });


    println!("Hello, world!");
}


fn process_item(_: usize) {

    thread::sleep(Duration::from_millis(37));
}