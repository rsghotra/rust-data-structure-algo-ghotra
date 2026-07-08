use std::collections::VecDeque;

fn main() {

    println!("========== Creating VecDeque ==========\n");
    
    /*
        - VecDeq::new()
        - VecDeq::from(vec![]);
        - collection::<VecDeq<_>>()
        - VecDeq::with_capacity(5)
    
    */

    let mut q1 = VecDeque::new();

    q1.push_back(10);
    q1.push_back(20);

    println!("q1 = {:?}\n", q1);

    //2. vecdeq with capacity

    let mut q2 = VecDeque::with_capacity(5);
    q2.push_back(2);
    q2.push_back(3);

    println!("q2 = {:?}", q2);
    println!("q2 capacity = {}\n", q2.capacity());

    //3. from array
    let q3 = VecDeque::from([20, 30, 40]);
    
    println!("q3 = {:?}\n", q3);


    //4. VecDeque from Vec using collect

    let v = vec![10, 30, 40, 50];

    //reference
    let q4 = v.iter().collect::<VecDeque<_>>();
    println!("q4 = {:?}\n", q4);

     //cloned/owned
    let q4_a = v.iter().cloned().collect::<VecDeque<_>>();
    println!("q4_A = {:?}\n", q4_a);
    
    //from a range

    let mut q5 = (1..=5).collect::<VecDeque<_>>();
    println!("q5 = {:?}\n", q5);


    println!("========== Adding Elements VecDeque ==========\n");

    /*
        push_front()
        push_back()
        append(&mut other): moves all element from other onto the back and other become empty 
        extend(iter): Add all elements from the iterator to the back
    
     */

    let mut q6 = VecDeque::new();
    q6.push_front(10);
    q6.push_back(20);


    println!("q6 = {:?}\n", q6);

    q6.append(&mut q5);
    println!("q6 after append = {:?}\n", q6);
    println!("q5 after append = {:?}\n", q5);

    q6.extend([7, 8]);
    println!("q6 after extend = {:?}\n", q6);

    println!("========== Removing Elements VecDeque ==========\n");

    /*
        pop_front(): Option<T>
        pop_back(): Option<T>
        remove(i): Option<T>
        retain(f): ()
        clear(): ()
     */

    let mut queue = VecDeque::from([10, 20, 30, 40, 50]);

    println!("pop_front()");

    if let Some(val) = queue.pop_front() {
        println!("Removed front = {:?}", val);
    }

    match queue.pop_back() {
        Some(val) => println!("Removed back = {:?}", val),
        None => println!("Ring buffer is empty"),
    }

    match queue.remove(10) {
        Some(val) => println!("Removed from index 2, value = {:?}", val),
        None => println!("Index out of bound"),
    }

    queue.retain(|x| *x %2 == 0);

    println!("retain()");

    println!("{:?}", queue);

    queue.clear();

    println!("clear()");

    println!("{:?}", queue);


    println!("========== Accessing / Peeking ==========\n");
    /*
        get(i) => Option<&T>
        get_mut(i) => Option<&mut T>
        front() => Option<&T>
        front_mut() => Option<&mut T>
        back() => Option<&T>
        back_mut() => Option<&mut T>
        len()
        is_empty()
     */

    let mut queue = VecDeque::from([10, 20, 30, 40]);

    //1. get(i)

    
    println!("get()");

    println!("Index 2 = {:?}", queue.get(2));

    println!("Index 10 = {:?}\n", queue.get(10));

    // get_mut(i)

    if let Some(value) = queue.get_mut(2) {
        *value += 25;
    }

    println!("{:?}\n", queue);

    //3. front()
    println!("front()");

    println!("{:?}", queue.front());

    println!("{:?}\n", queue);

    //4. front_mut()
    if let Some(val) = queue.front_mut() {
        *val += 10;
    }

    println!("front_mut()");

    println!("{:?}\n", queue);

    println!("back()");

    println!("{:?}", queue.back());

    println!("{:?}\n", queue);

    //------------------------------------------------------
    // 6. back_mut()
    //------------------------------------------------------
    // Returns:
    // Option<&mut T>

    if let Some(back) = queue.back_mut() {
        *back += 100;
    }

    println!("back_mut()");

    println!("{:?}\n", queue);

    println!("len = {:?}", queue.len());

    println!("========== VecDeque Iteration ==========\n");

    for value in queue.iter() {
        println!("{}", value);
    }

    for value in queue.iter_mut() {
        *value += 10;
    }

    println!("{:?}\n", queue);

    for value in queue.into_iter() {
        println!("{:?}\n", value);
    }

    //println!("{:?}\n", queue);

    println!("========== VecDeque Searching & Sorting ==========\n");
    /*    
        - contains()
        - iter().position()
        - iter().find()
        - make_contiguous().sort()
     */

    let mut queue = VecDeque::from([5, 10, 40, 20, 30]);

    println!("contains()");

    println!("Contains 20 ? {}", queue.contains(&20));

    println!("Contains 100 ? {}\n", queue.contains(&100));

    //find the fist element matching the predicate

    let result = queue.iter().find(|x| **x > 25);
    println!("result fron find: {:?}", result);

    let pos = queue.iter().position(|x| *x > 30);
    println!("result fron position: {:?}", pos);

    queue.make_contiguous().sort();

    println!("{:?}\n", queue);

}
