use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::rc::Rc;

fn main() {
    //simple thread spawning
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("The number {i} from the spawn thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("The number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    //moving data to spawned thread via move keyword
    let v = vec![1,2,3,4];

    let handle = thread::spawn(move || {
        println!("Here is the vector: {:?}", v);
    });

    handle.join().unwrap();


    //Channels: transferring data between threads using mpsc
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        let val2 = String::from("hhee");
        tx.send(val).unwrap(); //returns Result<T, E> //unwrap will panic if receiver has dropped already
        tx.send(val2).unwrap();
        //println!("Val 2 is {}", val2);

    });

    //when transmitter closes, receiver will return Error to signal no more data will be coming
    let received = rx.recv().unwrap(); //will block main thread execution
    println!("Got: {received}");

    //multiple producer + single consumer

    let (tx1, rx1) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Got: {received}");
    }

    /*
        shared concurrency
        Arc<Mutex<T>>
        Other Shared Memory Primiives
        - Mutex
        - RwLock
        - Atomics
        - Lockfree Structure
        - Spinlock
        - Condition Variables(Condvar)
        - Semaphores
        - Bridge
        - Memory Ordering
            - Acquired
            - Released
            - Relaxed
        

        non-shared concurrency
        - move
        - channels
            - mpsc
            - singleshot
            - broadcast
            - watch
            - notify
        
        Threads Related Terms
            - std::thread
            - tokio::thread

     */

    let m = Mutex::new(5);
    {
        
        let mut num = m.lock().unwrap();
        //MutxeGuard type implements DeRef and Drop(which drops the lock when num goes outof scope)
        *num = 6;
    }

    println!("m = {m:?}");

    //multithreaded Arc<Mutex<T>>

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result = {}", *counter.lock().unwrap());

}

