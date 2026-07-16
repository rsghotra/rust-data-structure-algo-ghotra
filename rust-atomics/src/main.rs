use std::collections::VecDeque;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::{Cell, RefCell};
use std::time::Duration;
use std::thread::park;
use std::sync::Condvar;

fn main() {

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread!");
    
    println!("Hello, world!");

    match t1.join() {
        Ok(out) => println!("thread has joined {:?}", out),
        Err(_) => println!("Error found")
    }

    t2.join().unwrap();

    /*
        passing closure to the thread instead of function
    
     */

    let numbers = vec![1, 2, 3];

    //spawn has a 'static lifetime bound on its argument type

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();


    //returning value from the clousre


    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum/len
    }).join();

    match t {
        Ok(average) => println!("average: {average}"),
        Err(e) => println!("Error: {:?}", e)
    }


    /*
        scoped threads: if we know for sure that spawned thread will not outlive a certain scope, that thread
        could safely borrow things that do not live forever, such as local variables as long as they outlive the scope

        thread::scope function --> allow to spawn threads on its scoped object

        No move required 
        It allows us to spawn threads that cannot outlive the scope of the closure(s) we pass to that function, making it possible to safely borrow local variables.
     */

    let numbers = vec![1,2,3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("lenthth: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
    //when scope s ends - all threads that haven’t been joined yet are automatically joined. Essentialy there is an invisible Join here
    //this scope spawn method does not have 'static bound on its argument type

    //right now both threads are reading no issue but when i try to modify the vectore using one of the soped thread

    // thread::scope(|s| {
    //     s.spawn(|| {
    //         numbers.push(5);
    //     });
    //     s.spawn(|| {
    //         numbers.push(12);
    //     });
    // });

    static  X: [i32; 3] = [1, 2, 3];

    thread::spawn(move || dbg!(&X));
    thread::spawn(move || dbg!(&X));

    //sharing ownership by leaking an allocation. Using Box::leak, one can release ownership of a box, promising it to never drop it

    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));


    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); //same allocation

    // move b to another thread(BUT Rc is not Send + Sync & not suitable for threads)

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a)).join().unwrap();
    thread::spawn(move || dbg!(b)).join().unwrap();

    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
            
        }
    }).join().unwrap();

    let index = 2;


    let a = [ 123, 456, 789];
    let b = unsafe {
        a.get_unchecked(index)
    };

    println!("{b}");


    /*
    
        Interior mutability:

        1. cell<T>: allow copy or replace the value as a whole - single threaded
    
     */
    let a = Cell::new(2);
    let b = Cell::new(3);

    f_1(&a, &b);

    let c = Cell::new(vec![0, 1, 2]);

    f_2(&c);

    let v = c.take();
    println!("vec inside a Cell: {:?}", v);
    c.set(v);

    assert_eq!(c.into_inner(), vec![0, 1, 2, 1]);
    

    //refcell
    let v = RefCell::new(vec![1, 2, 3]);
    f_3(&v);
    assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);



    let m = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = m.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    //Lock Poisoning
    assert_eq!(m.into_inner().unwrap(), 1000);

    let m = Mutex::new(vec![1, 2, 3]);

    m.lock().unwrap().push(2);

    println!("vector: {:?}", m);

    if let Some(item) = m.lock().unwrap().pop() {
        process_item(item);
    }

    let item = m.lock().unwrap().pop();

    if let Some(item) = item {
        process_item(item);
    }


    // /*
    //     thread::park
    //  */

    // let queue = Mutex::new(VecDeque::new());


    // thread::scope(|s| {
    //     //consumer thread
    //     let t = s.spawn(|| loop {
    //         let item = queue.lock().unwrap().pop_front();

    //         if let Some(item) = item {
    //             dbg!(item);
    //         } else {
    //             thread::park();
    //         }

    //     });

    //     //producing thread

    //     for i in 0.. {
    //         queue.lock().unwrap().push_back(i);
    //         t.thread().unpark();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });


    //ConditionalVariable: notify_one, notify_all

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();

                let item: i32 = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });

}


fn process_item(x: i32) {
    let y = x*2;
    println!("{y}");
}


fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();

    println!("This is my thread id: {:?}", id);
}

fn f_1(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();

    b.set(b.get() + 1);
    let after = a.get();

    println!("After: {after}");
    println!("After_b: {b:?}");

    if before != after  {
        println!("Mutability did not occur");
    }
}

fn f_2(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); //replaces the content of the cell with an empty vec
    v2.push(1);
    v.set(v2);
}

fn f_3(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
