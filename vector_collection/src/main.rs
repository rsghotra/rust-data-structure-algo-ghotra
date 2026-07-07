/*
Chapter 1 - Vec


A Vec<T> is Rust's growable array

Properties:
- Stored on the heap
- O(1) random access
- O(1) amortized push_back
- Can grow/shring dynamically

*/

#[derive(Debug)]
struct Order {
    id: u32,
    price: u32,
    quantity: u32,
}

fn main() {
    /*
       Patch 1: Creation
       Methods:
       1 Vec::new(): Empty, no allocation until first push
       2 vec![...] macro: Most common - literal content
       3 vec![val; n]: let v = vec![0; 5] - repeat a value n times
       4 Vec::with_capacity(n): let mut v = Vec::with_capacity(100); - pre allocates - avoid reallocation if final size is roughly known
       5 Vec::from(array): let v = Vec::from([1,2,3]); - From a fixed size array
       6 collect::<Vec<_>>(): let v = (1..5).collect::<Vec<_>>(); - Build from any iterator
       7 clone: let v = v1.clone() - clone another iterator
    */

    println!("=============== Creating Vectors ===============\n");

    //type #1 : Empty vector, type inferred later
    let mut v1 = Vec::new();
    v1.push(10);
    v1.push(20);

    println!("v1 = {:?}", v1);

    // vec macro
    let v2 = vec![1, 2, 3, 4, 5];

    println!("v2 = {:?}", v2);

    //vector of strings
    let fruits = vec!["Apple", "Orange", "Banana"];
    println!("fruits = {:?}", fruits);

    //vector with repeated values
    let zeroes = vec![0; 5];

    println!("Zeroes = {:?}", zeroes);

    let mut prices = Vec::with_capacity(5);
    prices.push(15);
    prices.push(20);

    println!("Prices = {:?}", prices);
    println!("Capacity = {:?}", prices.capacity());
    println!("Length = {:?}", prices.len());

    //from array
    let arr = [10, 20, 30, 40];

    let v3 = Vec::from(arr);
    println!("v3 = {:?}", v3);

    //from iterator
    let v4 = (1..=5).collect::<Vec<_>>();

    println!("v4 = {:?}", v4);

    //clone
    let v5 = v4.clone();
    println!("v5 = {:?}", v5);

    //Iterator + map
    let squares = (1..=5).map(|x| x * x).collect::<Vec<_>>();

    println!("squares = {:?}", squares);

    /*
       Accessing: Vec<T> supports fast random access using the following methods

       1 Unsafe Indexing: v[i] --> can panic if index given is out of bound; Returns - T; let x = v[2];
       2 Safe Read Access: v.get(i); Access index without panic even if out of bound given; Returns Option<&T>
       3 Safe Mutable Access: v.get_mut(i); access index safely and if available returns a mutable access to the index for changing; Returns Option<&mut T>
       4 Safe first element access: v.first(); access first index if available; Returns Option<&T>
       5 Saft last element access: v.last(); access last index element if available; Returns Option<&T>
       6 Mutable safe first element access: v.first_mut(); Returns Option<&mut T>
       7 Mutable safe last element access: v.last_mut(); Returns Option<&mut T>
       8. Unsafe slice: &v[1..3]: returns the slice(last element exclusive); panic if given out of range; Returns &[T]
       9. Safe slice: : v.get(1..3) returns the slice(last element exclusive); Returns Option<&[T]>

       1. Indexing:        v[0]
       2. Safe access:    v.get(0)
       3. Immutable and Mutable First element Access:  v.first() or v.first_mut()
       4. Immutable and Mutable Last element Access:   v.last() or v.last_mut()
       5. Unsage and Safe Slices:         &v[1..3]; v.get(1..3) and btw no allocation happens
       6. Iteration:      v.iter()

    */

    println!("==============Access Vec Elements=================");

    let mut prices = vec![100.0, 200.0, 300.30, 99.8, 102.40];

    //------------------------------------------------------
    // 1. Access by index
    //------------------------------------------------------
    // Fast, but panics if index is out of bounds.

    let first_price = prices[0];
    println!("First price using index = {}", first_price);

    /*
       Safe access using get()
       Safer than indexing
    */

    match prices.get(1) {
        Some(val) => println!("Second price using get = {:?}", val),
        None => println!("No price found at index 1"),
    }

    /*
       Safe access using get()
       Out of bound access using get(i) as it returns Option<&T>
    */

    match prices.get(10) {
        Some(val) => println!("Tenth price using get = {:?}", val),
        None => println!("Out of bound value giving but no panic"),
    }

    //with if let i can actually avoid using full match statements
    //first immutable element
    if let Some(value) = prices.first() {
        println!("Value found: {}", value);
    }

    //now mutable first element - two ways, using get and first_mut
    if let Some(value) = prices.get_mut(0) {
        *value = 101.0;
    }

    println!("Prices: {:?}", prices);

    match prices.first_mut() {
        Some(val) => {
            *val = 102.46;
        }
        None => println!("Out of bound values found"),
    }

    println!("Prices: {:?}", prices);

    if let Some(value) = prices.last_mut() {
        *value = 205.1;
    }

    println!("Prices: {:?}", prices);

    let output = prices.get(1..3);

    if let Some(output) = prices.get(2..3) {
        println!("Output[0]: {:?}", output.get(0));
    }

    println!("Output from slice = {:?}", Some(output));

    for price in prices.iter() {
        println!("Price = {:?}", price);
    }

    for price in prices.iter_mut() {
        *price += 10.5;
    }

    println!("New Prices = {:?}", prices);

    let prices = prices.iter_mut().map(|x| *x * 2.0).collect::<Vec<_>>();

    println!("New Prices = {:?}", prices);

    println!("========== Adding Elements ==========\n");

    /*
       push : add an element to end of vector: v.push(3);
       insert: insert before the specfied index. All elements after the index are shifted
       append: append other vector to the end of the given vector by MOVING all elements, append vector becomes empty; takes mutable reference
       extend: extend current vector from other VECTOR ITERATOR: Other iterator remain valid
       resize: resize the vector and fill with the supplied value
    */

    let mut numbers = vec![10, 20, 30];

    println!("Initial Vector = {:?}", numbers);

    numbers.push(40);

    numbers.insert(1, 15);

    println!("Modified Vector = {:?}", numbers);

    let mut other = vec![50, 60, 70];

    //append

    numbers.append(&mut other);

    println!("Appended Vector = {:?}", numbers);

    //extend(iter)

    numbers.extend(other.iter().clone()); //see other vector already became empty due to append;

    let extra = vec![80, 90];

    numbers.extend(extra.iter().clone());

    println!("Real Extended Vector = {:?}", numbers);
    numbers.extend(vec![75, 85]);
    println!("Real Extended Vector x 2 = {:?}", numbers);
    numbers.resize(20, 100);

    println!("Resized Vector = {:?}", numbers);

    println!("========== Removing Elements ==========\n");
    /*
        - pop(): Remove last; Option<T> = x.pop()
        - remove(): Not safe remove and panics; If valid then remove element from asked index and return
        - swap_remove(): interesting, O(1) removal; It swaps the element to be removed with the last element of the vector then pops the last element
        - retain(): different from filter. It works on the original vector compare to filter() which works on iterator and is an iterator adaptor means produces a new vector
        - retain_mut(): same as retain() but gets a &mut reference which allows the elements to be modified and then the criteria of drop could be applied
        - drain()
        - clear()
        - truncate()

    */

    //pop()

    let mut v = vec![10, 20, 30, 40];

    println!("Original = {:?}", v);

    let popped = v.pop();

    println!("Popped element = {:?}", popped);
    println!("Vector = {:?}", v);

    //removed by index but not safe and panics as below
    // let removed = v.remove(10);
    // println!("Removed element = {:?}", removed);
    // println!("Vector = {:?}", v);

    //removed from valid index
    let removed = v.remove(1);
    println!("Removed element = {:?}", removed);
    println!("Vector = {:?}", v);

    v.append(&mut vec![50, 60, 70]);

    println!("Vector = {:?}", v);
    //swap_remove(i);

    let swap_removed = v.swap_remove(2); //use when order doies not matter

    println!("Swap Removed element = {:?}", swap_removed);
    println!("Vector = {:?}", v);

    //retain
    let mut v = vec![1, 2, 3, 4, 5];

    //retain: mutates vector directly and shrinks it in place
    v.retain(|x| x % 2 == 0);
    println!("{:?}", v); //[2, 4] - v itself is now this

    //retain_mut
    let mut v = vec![1, 2, 3, 4, 5];

    v.retain_mut(|x| {
        *x += 10; // mutate first
        *x % 2 == 0 //then decide: keep if now even
    });

    let mut v = vec![1, 2, 3, 4, 5];

    //drain of a slice
    for x in v.drain(1..3) {
        println!("x: {:?}", x);
    }

    //clear() - capacity remains
    let mut v = vec![1, 2, 3, 4, 5];

    v.clear();

    println!("After clear() = {:?}", v);
    println!("Capacity = {}\n", v.capacity());

    //truncate() - keep only first N elements

    let mut v = vec![1, 2, 3, 4, 5];

    v.truncate(3);

    println!("After truncate() = {:?}", v);

    println!("========== Updating Elements ==========\n");

    /*
       v[i]
       swap(i, j)
       get_mut(i):
       first_mut(i):
       last_mut(i):
       iter_mut(i):
       fill(val):
       copy_from_slice()

    */
    let mut v = vec![10, 20, 30, 40];

    v.swap(1, 3);

    println!("After swap() = {:?}", v);

    if let Some(value) = v.first_mut() {
        *value += 10;
    }

    println!("After swap() = {:?}", v);

    v.fill(10);
    println!("After fill() = {:?}", v);

    println!("========== Capacity Management ==========\n");

    /*
       Capacity:
       with_capacicty
       len()
       capacity()
       is_empty()
       reserve(): reserve at least this much addition bits
       reserve_exact()
       shrink_to_fit()
       strink_to()
       try_reserve()
       try_reserve_exact()

    */

    let mut numbers = Vec::with_capacity(10);

    numbers.push(10);
    numbers.push(20);

    println!("len()      = {}", numbers.len());
    println!("capacity() = {}", numbers.capacity());
    println!("is_empty() = {}", numbers.is_empty());

    numbers.reserve(20);

    println!("After reserve(20)");
    println!("capacity = {}", numbers.capacity());

    numbers.reserve_exact(20);

    println!("After reserve_exact(20)");
    println!("capacity = {}", numbers.capacity());

    numbers.shrink_to_fit();
    println!("After shrink_to_fit()");
    println!("capacity = {}", numbers.capacity());

    numbers.reserve(20);

    println!("\nAfter reserve(20)");
    println!("capacity = {}", numbers.capacity());

    numbers.shrink_to(5);

    println!("After shrink_to(5)");
    println!("capacity = {}", numbers.capacity());

    println!("========== Searching ==========\n");

    /*
        contains():
        iter().find(): return value matching predicate
        position(): return index matching predicate
        rposition(): returns last matching index [10, 20, 30, 40, 20] =>4
        binary_search():
        binary_search_by():
        binary_search_by_key()
        starts_with()
        ends_with()

    */

    let numbers = vec![10, 20, 30, 40, 50];

    //1. contains
    println!("Contains 30? {}", numbers.contains(&30));
    println!("Contains 60? {}", numbers.contains(&60));

    //find() or iter.find() - remember ** Find First element containing matching value; Option<&T>; returns the value not index

    let result = numbers.iter().find(|x| **x > 25); //consuming iterator

    println!("First element > 25 = {:?}", result);

    //position returns Option<usize>: interesting since index it is a usize

    let index = numbers.iter().position(|x| *x == 40);

    println!("Position of 40 = {:?}", index);

    //rposition, returns last matching index
    let duplicates = vec![10, 20, 30, 40, 20, 20, 10];

    let last = duplicates.iter().rposition(|x| *x == 20);

    println!("Last position of 20 = {:?}\n", last);

    //binary_seach == vector must be sorted

    match numbers.binary_search(&70) {
        Ok(i) => println!("Value found at {i}"),
        Err(err) => eprintln!("Value not found, insert at index {}", err),
    }

    println!("========== Vec Sorting ==========\n");

    /*

            ===========================================================

    Complexity

    sort()                   : O(n log n)
    sort_unstable()          : O(n log n)
    sort_by()                : O(n log n)
    sort_unstable_by()       : O(n log n)
    sort_by_key()            : O(n log n)
    sort_unstable_by_key()   : O(n log n)
    reverse()                : O(n)

    ===========================================================

    Key Difference

    sort()
    - Stable
    - Equal elements keep relative order
    - Slightly more overhead

    sort_unstable()
    - Not stable
    - Equal elements may reorder
    - Usually faster
    - Good default when equal-order does not matter

    ===========================================================

    Common Interview Traps

    1. Descending sort

    Use:

    values.sort_by(|a, b| b.cmp(a));

    Not:

    values.sort_by(|a, b| a.cmp(b));  // ascending

    -----------------------------------------------------------

    2. sort_by_key() is clean for structs

    orders.sort_by_key(|order| order.price);

    -----------------------------------------------------------

    3. Stable vs unstable

    If equal elements have important secondary meaning, use sort().

    Example:
    - Same price orders
    - Earlier order should remain before later order
    - Price-time priority systems

    Then stable sort matters.

    -----------------------------------------------------------

    4. For trading systems

    For order books:
    - Bids usually sorted descending by price
    - Asks usually sorted ascending by price

    bids.sort_by(|a, b| b.price.cmp(&a.price));
    asks.sort_by(|a, b| a.price.cmp(&b.price));

    ===========================================================

    Interview Notes

    Use sort_unstable() when:
    - You only care about final sorted values
    - Equal-order does not matter
    - You want faster sorting

    Use sort() when:
    - Equal values must preserve insertion order
    - Stable ordering matters

    Use sort_by() when:
    - You need custom ordering
    - Descending order
    - Multi-field comparison

    Use sort_by_key() when:
    - Sorting by one simple field

    ===========================================================

    */

    //1. sort() - sorts in ascending order + stable sort : maintain order of equal elements
    let mut numbers = vec![50, 10, 40, 30, 20, 30];

    numbers.sort();

    println!("After sort() = {:?}", numbers);

    let mut numbers = vec![50, 10, 40, 30, 20, 30];

    numbers.sort_unstable();

    println!("After sort_unstable() = {:?}", numbers);

    //sort_by(cmp)

    let mut prices = vec![100, 105, 99, 110];

    prices.sort_by(|a, b| a.cmp(b)); //ascending order

    println!("After sort_by() = {:?}", prices);

    prices.sort_by(|a, b| b.cmp(&a));
    println!("After sort_by() = {:?}", prices);

    let mut orders = vec![
        Order {
            id: 1,
            price: 105,
            quantity: 10,
        },
        Order {
            id: 2,
            price: 100,
            quantity: 20,
        },
        Order {
            id: 3,
            price: 110,
            quantity: 5,
        },
    ];

    orders.sort_by_key(|order| order.price);

    println!("Orders sorted by price ascending:");
    for order in &orders {
        println!("{:?}", order);
    }

    orders.sort_unstable_by_key(|order| order.quantity);

    println!("\nOrders sorted by quantity ascending:");
    for order in &orders {
        println!("{:?}", order);
    }

    //reverse
    let mut values = vec![1, 2, 3, 4, 5];

    values.reverse();

    println!("\nAfter reverse() = {:?}", values);

    println!("\nis_sorted() = {:?}", values.is_sorted());

    println!("========== Vec Iteration ==========\n");

    /*

       iter()
       iter_mut():
       into_iter(): moves ownership - vector is gown
       enumerate(): if need indices
       zip(): traverse two collections together
       for_each()
       by_ref()
       cloned()
       copied()
       peekable()
    */

    let mut numbers = vec![10, 20, 30];

    println!("iter()");

    for value in numbers.iter() {
        println!("{value}");
    }

    println!("numbers = {:?}\n", numbers);

    for value in numbers.iter_mut() {
        *value += 10;
    }

    println!("numbers = {:?}\n", numbers);

    for value in numbers.into_iter() {
        println!("{value}");
    }

    //println!("numbers = {:?}\n", numbers); //vector numbers moved

    let mut numbers = vec![100, 200, 300];

    println!("\nenumerate()");

    for (i, num) in numbers.iter().enumerate() {
        println!("{:?} -> {:?}", i, num);
    }

    //zip
    let prices = vec![100, 101, 102];
    let qty = vec![5, 10, 15];

    println!("\nzip()");

    for (p, q) in prices.iter().zip(qty.iter()) {
        println!("Price {} Qty {}", p, q);
    }

    //for_each
    println!("\nfor_each()");

    //inpace modification

    numbers.iter().for_each(|x| println!("{x}"));

    numbers.iter_mut().for_each(|x| *x += 1);

    numbers.iter().for_each(|x| println!("{x}"));

    let words = vec![String::from("Rust"), String::from("Trading")];

    let _cloned = words.iter().cloned().collect::<Vec<_>>();

    println!("\ncloned()");

    println!("{:?}", _cloned);

    let values = vec![10, 20, 30];

    let copied = values.iter().copied().collect::<Vec<_>>();

    println!("\ncopied()");

    println!("{:?}", copied);

    println!("========== Slicing & Partitioning ==========\n");

    /*
    1. split_at(): splits a slice into two immutable slices: &[T]( this means immutable slice reference)
    2. split_at_mut(): splits a slices into two mutable slices: &mut [T]
    3. chunks(): gives non overlapping chunks but immutable: &[T]
    4. chunks_mut(): gives non overlapping chunks and mutable: &mut [T]
    5. windows(): gives overlapping windows: &[T]
    6. chunks_exact(): gives non overlapping chunks and leftover gets dropped: &[T]

    */

    //1. split_at: gives two immitable slices

    let numbers = vec![10, 20, 30, 40, 50, 60];

    //&[i32] is known as vector slice
    let (left, right) = numbers.split_at(3);

    println!("split_at()");
    println!("Left  = {:?}", left);
    println!("Right = {:?}\n", right);

    //2. split_at_mut

    let mut numbers = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = numbers.split_at_mut(3);

    left[0] = 100;
    right[0] = 400;

    println!("split_at_mut()");
    println!("{:?}\n", numbers);


    //chunks
    let mut numbers = vec![1,2,3,4,5,6,7];

    println!("chunks()");

    for chunk in numbers.chunks(3) {
        println!("{:?}", chunk);
    }

    //chunks_mut
    for chunk in numbers.chunks_mut(2) {
        println!("{:?}", chunk);
        chunk[0] += 10;
        println!("{:?}", chunk);
    }

    println!("\nchunks_mut()");

    println!("{:?}", numbers);

    //------------------------------------------------------
    // 5. windows()
    //------------------------------------------------------
    // Overlapping windows.

    let numbers = vec![10,20,30,40];

    println!("\nwindows()");

    for window in numbers.windows(2) {
        println!("{:?}", window);
    }

     //------------------------------------------------------
    // 6. chunks_exact()
    //------------------------------------------------------
    // Drops incomplete final chunk.

    let values = vec![1,2,3,4,5,6,7];

    println!("\nchunks_exact()");

    for chunk in values.chunks_exact(3) {
        println!("{:?}", chunk);
    }
    
}
