use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc}; //using standard libraries locks
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);

    let producer = thread::spawn(move || {
        let mut queue = queue_producer.lock().unwrap();
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);

        println!("All values pushed");
        thread::sleep(Duration::from_secs(5)); //producer is holding on the lock
    });

    let consumer = thread::spawn(move || {
        let mut queue = queue_consumer.lock().unwrap();
        while let Some(item) = queue.pop_front() {
            println!("Popped: {}", item);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    //Segueue == unbounded multiproducer and multi consumer
    //ArrayQueue == bounded multiproducer and multi consumer
    let queue = Arc::new(SegQueue::new());
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);

    let producer = thread::spawn(move || {
        queue_producer.push(1);
        queue_producer.push(2);
        queue_producer.push(3);

        println!("All values pushed");
        thread::sleep(Duration::from_secs(5)); //producer is holding on the lock
    });

    let consumer = thread::spawn(move || {
        while let Some(item) = queue_consumer.pop() {
            println!("Popped: {}", item);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    //channels allow to have better control over who is consuming data 
    // no join management either

    //sync channel are bounded
    let (tx, rx) = mpsc::sync_channel(2);

    let tx1 = tx.clone();

    thread::spawn(move || {
        tx1.send("msg1").unwrap();
        println!("Sent msg1");

        tx1.send("msg2").unwrap();
        println!("Sent msg2");

        tx1.send("msg3").unwrap();
        println!("Sent msg3");
    });

    
    for received in rx.iter().take(3) {
        println!("Main received: {}", received);
    }

}
