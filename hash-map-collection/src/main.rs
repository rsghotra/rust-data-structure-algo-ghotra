use std::collections::HashMap;

fn main() {
    println!("========== Creating HashMap ==========\n");
    /*
      1. HashMap::new() = empty hashmap, no allocation until first insert
      2. HashMap::from([...]) = Build from array of tuples - clean literal syntax
      3. collect::<HashMap<_>>() = useful when transforming from Vec<(K, V)> into a map or from any iterator with K, V pair
      4. HashMap::with_capacity(n) = Important in system code to reduce allocation
    */

    //1 Empty hashmap

    let mut prices = HashMap::new();

    prices.insert("EUR/USD".to_string(), 1.10);
    prices.insert("USD/JPY".to_string(), 150.0);

    println!("prices: {:?}\n", prices);

    let mut prices: HashMap<&str, f64> = HashMap::new();

    prices.insert("BTC", 2.2);
    prices.insert("ETH", 5.5);

    println!("prices: {:?}\n", prices);

    //2 Hashing from array slices

    let pairs = vec![("BTC/USD", 20000), ("ETH/USD", 5000)];

    let crypto_pairs = HashMap::from(pairs.into_iter().collect::<HashMap<_, _>>());
    println!("crypto_pairs: {:?}", crypto_pairs);

    let equity_pairs = HashMap::from([("NVDA/USD", 193.50), ("OKLO/USD", 4.50)]);
    println!("Equity pairs: {:?}", equity_pairs);

    let pairs = vec![("BTC/USD", 20000), ("ETH/USD", 5000)];

    let collected = pairs.into_iter().collect::<HashMap<_, _>>();
    println!("Collected = {:?}", collected);

    println!("========== Accessing Values ==========\n");

    /*
       Accessing values:

       1. get(&key): get(&key) => Option<&V>
       2. get_mut(&key): get_mut(&key) => Option<&mut V>
       3. contains_key(&key): bool
       4. Index Operator []: map[&key] => V | will panic if key do not exist
       5. get_key_value(&key) => Option<(&K, &V)>


    */

    let mut prices = HashMap::from([("BTC", 65000), ("ETH", 3000), ("SOL", 150)]);

    prices.insert("ADA", 312);

    //1. get()

    match prices.get("BTC") {
        Some(price) => println!("BTC price found: {:?}", price),
        None => println!("BTC not found"),
    }

    //2 get_mut

    if let Some(price) = prices.get_mut("BTC") {
        *price += 100;
    }

    println!("\nAfter get_mut()");

    println!("{:?}", prices);

    //3. contains_key() --> returns bool

    println!("\ncontains_key()");

    println!("Contains BTC? = {:?}", prices.contains_key("BTC"));
    println!("Contains XRP? = {:?}", prices.contains_key("XRP"));

    //4. access by index

    println!("Value of BTC: {:?}", prices["BTC"]);

    //panic
    //println!("Value of BTC: {:?}", prices["XRP"]);

    //get_key_value(&K) => Option<(&K, &V)>

    match prices.get_key_value("SOL") {
        Some((symbol, price)) => {
            println!("{} -> {}", symbol, price);
        }
        None => println!("Not found"),
    }

    println!("========== Entry API ==========\n");

    /*  f == means closure
       entry(): returns Entry enum Entry::Occupied & Entry::Vacant
       entry(&K).or_insert(v): insert ONLY if key DOES NOT exist => &mut V
       entry(&K).and_modify(f): Update ONLY if key DOES exists else no op
       entry(&K).and_modify(f).or_insert(v): classic update if exists or set default
       entry(&K).or_default(): if key does not exist, create and set default value

    */

    //entry

    let mut prices: HashMap<&str, i64> = HashMap::new();

    let entry = prices.entry("BTC");

    println!("Entry = {:?}", entry);

    //or_insert_key()

    prices.entry("BTC").or_insert(65_000);

    println!("After first or_insert()");
    println!("{:?}\n", prices);

    prices.entry("BTC").or_insert(70_000);

    println!("After second or_insert()");
    println!("{:?}\n", prices);

    //insert
    prices.insert("ETH", 1350);

    //and_modify: only modify if value existed
    prices.entry("BTC").and_modify(|price| *price += 500);

    println!("After and_modify() when BTC existsed");
    println!("{:?}\n", prices);

    //and_modify when value do not exist - no action happen
    prices.entry("SOL").and_modify(|price| *price += 200);

    println!("After and_modify() when SOL did not existed");
    println!("{:?}\n", prices);

    //and_modify().or_insert(): update if value exists else set default

    prices
        .entry("SOL")
        .and_modify(|price| *price = 150)
        .or_insert(0);

    println!("After and_modify(f).or_insert(v) when SOL did not existed");
    println!("{:?}\n", prices);

    prices
        .entry("SOL")
        .and_modify(|price| *price = 150)
        .or_insert(0);

    println!("After 2nd and_modify(f).or_insert(v) when SOL did not existed");
    println!("{:?}\n", prices);

    //_or_default()

    let mut positions: HashMap<&str, Vec<i32>> = HashMap::new();

    positions.entry("Alice").or_default();

    println!("After 1st .or_default");
    println!("{:?}\n", positions);

    positions.entry("Alice").or_default().push(100);

    println!("After 2nd .or_default");
    println!("{:?}\n", positions);

    println!("========== Removing Elements ==========\n");

    /*
       1. remove(&key) => Option<V>
       2. remove_entry(&key) => Option<(K, V)>
       3. retain(f) => () => keep only entries matching predicate over (&K, &mut V)
       4. clear()

    */

    let mut prices = HashMap::from([("BTC", 65000), ("ETH", 3000), ("SOL", 150), ("DOGE", 1)]);

    println!("remove()");

    let removed = prices.remove("DOGE");
    println!("Removed = {:?}", removed);

    println!("{:?}\n", prices);

    println!("remove_entry()");

    match prices.remove_entry("ETH") {
        Some((key, value)) => println!("Removed {:?} -> {:?}", key, value),
        None => println!("Key not found"),
    }

    println!("{:?}\n", prices);

    //retain
    println!("Retaining keys with value price > 60000");

    prices.retain(|_, price| *price > 60000);

    println!("{:?}\n", prices);

    //clearing
    prices.clear();

    println!("Cleared - {:?}\n", prices);

    println!("========== HashMap Iteration ==========\n");

    /*
       Task: Iteration
       1. iter() => (&k, &v)
       2. iter_mut() => (&k, &mut v)
       3. into_iter() => (k, v)
       4. keys() => &K
       5. values() => &V
       6. values_mut() => &mut V

    */

    let mut prices = HashMap::from([("BTC", 65000), ("ETH", 3000), ("SOL", 150)]);

    //1. iter

    println!("iter()");

    for (symbol, price) in prices.iter() {
        println!("{} -> {}", symbol, price);
    }

    //------------------------------------------------------
    // 2. iter_mut()
    //------------------------------------------------------
    // Produces:
    //
    // (&K, &mut V)

    println!("iter_mut()");

    for (_, price) in prices.iter_mut() {
        *price += 10;
    }

    println!("{:?}\n", prices);

    //------------------------------------------------------
    // 3. keys()
    //------------------------------------------------------
    // Produces:
    //
    // &K

    println!("keys()");

    for symbol in prices.keys() {
        println!("{}", symbol);
    }

    println!();

    //------------------------------------------------------
    // 4. values()
    //------------------------------------------------------
    // Produces:
    //
    // &V

    println!("values()");

    for price in prices.values() {
        println!("{}", price);
    }

    //------------------------------------------------------
    // 5. values_mut()
    //------------------------------------------------------
    // Produces:
    //
    // &mut V

    println!("values_mut()");

    for price in prices.values_mut() {
        *price *= 10;
    }

    println!("{:?}\n", prices);

    //find operation though not too useful in HashMap

    if let Some ((symbol, price)) = prices.iter().find(|(_, price)| **price > 1000) {
        println!("Expensive pair: {:?} -> {:?}", symbol, price)
    }

    //filter instead as find starts from beginning after every find

    for (symbol, price) in prices.iter().filter(|(_, price)| **price > 1000) {
        println!("Expensive pairs: {:?} -> {:?}", symbol, price);
    }

    //------------------------------------------------------
    // 6. into_iter()
    //------------------------------------------------------
    // Consumes the HashMap.
    //
    // Produces:
    //
    // (K, V)

    let prices = HashMap::from([("BTC", 65000), ("ETH", 3000)]);

    println!("into_iter()");

    for (symbol, price) in prices.into_iter() {
        println!("{} -> {}", symbol, price);
    }

    //prices can no longer be used after this
}
