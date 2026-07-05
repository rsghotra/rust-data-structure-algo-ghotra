use std::thread;
use std::sync::Arc;

trait Engine: Send + Sync {
    fn start(&self);
}

struct Car;
struct Bike;

impl Engine for Car {
    fn start(&self) {
        println!("Starting Car...");
    }
}

impl Engine for Bike {
    fn start(&self) {
        println!("Starting Bike...");
    }
}

fn main() {
    let x = 3;

    // we can pass to other thread so x is SEND
    
    thread::spawn(move || {
        println!("{}", x);
    })
    .join()
    .expect("Failed to Join");


    // if shared between multiple threads then its SYNC

    let y = 5;

    let handles: Vec<_> = (0..6)
    .map(|i| {
        thread::spawn(move || {
            println!("{i}");
        })
    })
    .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let vehicles: Vec<Box<dyn Engine>> = vec![Box::new(Car), Box::new(Bike)];
    //sending to single thread -- SEND
    vehicles.into_iter().for_each(|engine| {
        thread::spawn(move || {
            engine.start();
        })
        .join()
        .expect("threads failed");
    });

    //sending to multiple thread
    let car: Arc<dyn Engine> = Arc::new(Car);

    let handles: Vec<_> = (0..5)
    .map(|_| {
        let car_shared = Arc::clone(&car);
        thread::spawn(move || {
            car_shared.start();
        })
    }).collect();

    for handle in handles {
        handle.join().expect("thread failed");
    }

}
