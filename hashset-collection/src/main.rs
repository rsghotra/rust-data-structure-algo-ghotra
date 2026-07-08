use std::collections::HashSet;

fn main() {


    println!("========== Creating HashSet ==========\n");

    /*
        - HashSet::new()
        - HashSet::from([])
        - HashSet::with_capacity()
        - collection::<HashSet<_>>()
     */


    let mut set1 = HashSet::new();

    set1.insert("BTC");
    set1.insert("ETH");

    println!("new");

    println!("{:?}\n", set1);

    let mut set2 = HashSet::with_capacity(5);
    set2.insert(10);
    set2.insert(20);

    println!("{:?}\n", set2);

    //from: create from array, dups are removed

    let set3 = HashSet::from([
        "BTC",
        "ETH",
        "BTC",
        "SOL"
    ]);

    println!("{:?}\n", set3);


    //using collection iterator

    let symbols = vec!["BTC", "ETH", "XRP", "ADA", "SOL"];


    let mut set4 = symbols.into_iter().collect::<HashSet<_>>();
    println!("{:?}\n", set4);

    println!("========== Inserting & Removing ==========\n");

    println!("remove");

    if set4.remove("ETH") {
        println!("Remove of ETH successful");
    }

    match set4.take("BTC") {
        Some(val) => println!("Value removed: {:?}", val),
        None => println!("Value did not exist"),
    }

    println!("After 2 removals: {:?}\n", set4);

    match set4.replace("ADA") {
        Some(val) => println!("Value replaced: {:?}", val),
        None => println!("Value did not exist so did not replace"),
    }

    println!("After 1 replacement: {:?}\n", set4);

    let mut numbers = HashSet::from([1, 2,3,4,5,6]);

    numbers.retain(|x| *x % 2 == 0);

    println!("retain()");

    println!("{:?}\n", numbers);

    numbers.clear();

    println!("{:?}\n", numbers);

    println!("========== Accessing / Searching ==========\n");

    let set = HashSet::from([
        "BTC",
        "ETH",
        "SOL",
    ]);

    println!("contains()");

    println!("Contains BTC ? {}", set.contains("BTC"));

    println!("Contains XRP ? {}", set.contains("XRP"));

    println!();

    //get() --> return a reference to the stored object
    println!("get()");

    println!("{:?}", set.get("ETH"));

    println!("{:?}", set.get("DOGE"));

    println!();
    

     println!("len()");

    println!("{}", set.len());

    println!();

    //------------------------------------------------------
    // 4. is_empty()
    //------------------------------------------------------
    // Returns:
    // bool

    println!("is_empty()");

    println!("{}", set.is_empty());

    //difference: A - B

    let crypto = HashSet::from([
        "BTC",
        "ETH",
        "SOL",
    ]);

    let owned = HashSet::from([
        "BTC",
        "SOL",
    ]);

    println!("difference()");

    for coin in crypto.difference(&owned) {
        println!("{}", coin);
    }

    //intersection: Common elements: A and B
    println!("intersection");

    for coin in crypto.intersection(&owned) {
        println!("{}", coin);
    }

    //union: All unique elements
    let watchlist = HashSet::from([
        "DOGE",
        "ETH",
    ]);

    println!("union");

    for coin in crypto.union(&watchlist) {
        println!("{}", coin);
    }


}
