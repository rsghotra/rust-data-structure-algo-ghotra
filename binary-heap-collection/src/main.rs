use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Binary Heap by default in Max Heap
    //Heapfify takes On(n)
    /*
        Creating:
        BinaryHeap::new()
        BinaryHeap::from(vec)
        BinaryHeap::with_capacity()
        BinaryHeap collection
    
     */

    println!("========== Creating BinaryHeap ==========\n");
    let mut heap1 = BinaryHeap::new();

    heap1.push(10);
    heap1.push(20);
    heap1.push(30);

    println!("new()");

    println!("{:?}\n", heap1);

    let mut heap2 = BinaryHeap::with_capacity(10);

    heap2.push(20);
    heap2.push(40);

    println!("with_capacity()");

    println!("{:?}", heap2);

    println!("Capacity = {}\n", heap2.capacity());

    let heap3 = BinaryHeap::from([10, 50, 20, 80, 49]);

    println!("from()");

    println!("{:?}\n", heap3);

    //collect
    let numbers = vec![
        100,
        40,
        90,
        10,
    ];

    let heap4 = numbers.iter().collect::<BinaryHeap<_>>();

    println!("{:?}\n", heap4);

    println!("========== Adding & Removing ==========\n");

    let mut heap = BinaryHeap::new();

    heap.push(10);
    heap.push(20);
    heap.push(30);

    println!("push()");

    println!("{:?}\n", heap);

    //peek
    println!("peek: ");

    println!("{:?}", heap.peek());
    println!("{:?}", heap);

    println!("pop");

    if let Some(val) = heap.pop() {
        println!("Popped value: {:?}", val );
    }

    println!("peek_mut");

    if let Some(mut top) = heap.peek_mut() {
        *top += 10;
    }


    println!("{:?}\n", heap);

    //creating min heap
    println!("\n========== Min Heap ==========\n");

    let mut min_heap = BinaryHeap::new();

    min_heap.push(Reverse(30));
    min_heap.push(Reverse(80));
    min_heap.push(Reverse(20));
    min_heap.push(Reverse(50));

    println!("Heap: {:?}", min_heap);
    println!("Peek:{:?}", min_heap.peek());
    println!("Pop: {:?}", min_heap.pop());

     println!("========== Accessing / Searching ==========\n");

    let heap = BinaryHeap::from([
        40,
        10,
        80,
        30,
        50,
    ]);

    println!("len = {:?}", heap.len());

    
    let exists = heap.iter().any(|x| *x == 30);

    println!("{}", exists);

    let exists = heap.iter().find(|x| **x == 30).is_some();

    println!("{}", exists);

    println!("========== Conversion & Bulk Operations ==========\n");

    /*
        append(other heap)
        extend([..])
        into_iter(): Consumes the heap and order is NOT sorted
        into_sorted_vec(): Vec<T>: consumes the heap
        drain()

     */

    let heap = BinaryHeap::from([40, 10, 80, 30]);

    println!("into iter()");

    for value in heap.into_iter() {
        println!("{}", value);
    }

    //2. into sorted vec
    let heap = BinaryHeap::from([40, 10, 80, 30]);
    println!("into_sorted_vec()");
    let sorted = heap.into_sorted_vec();

    println!("{:?}\n", sorted);

    //3. extend from any iterator
    let mut heap = BinaryHeap::from([40, 10, 80, 30]);
    heap.extend([2,3,4,5]);

    println!("extend()");

    println!("{:?}\n", heap);

    let mut heap1 = BinaryHeap::from([10, 30]);

    let mut heap2 = BinaryHeap::from([50, 20]);

    heap1.append(&mut heap2);

    println!("append()");

    println!("heap1 = {:?}", heap1);

    println!("heap2 = {:?}\n", heap2);

    let mut heap = BinaryHeap::from([100, 40, 80]);

    println!("drain()");

    for value in heap.drain() {

        println!("{}", value);

    }

    println!("Heap after drain = {:?}", heap);

}
