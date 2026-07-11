mod cross_pair_v2;

/*
This version handles:

1. No error on graph creation for negative or 0.0 price. Instead, it will ignore the node and continue
2. Deals with staleness issue - and store hashmap with internal hashmap

*/

fn main() {
    let quotes = vec![
        ("EUR", "USD", 1.10),
        ("EUR", "GBP", 0.86),
        ("USD", "GBP", 0.78),
        ("USD", "CAD", 1.35),
        ("USD", "JPY", 145.0),
        ("USD", "CHF", 0.72),
        ("GBP", "CHF", 1.15),
        ("GBP", "JPY", 150.0),
        ("JPY", "AUD", 0.61),
    ];

    //graph will always be returned okay
    let graph = cross_pair_v2::create_cross_price_graph(&quotes);

    let base = "EUR";
    let quote = "JPY";

    match cross_pair_v2::derive_cross_price(&graph, base, quote) {
        Some(price) => println!("Derived cross price for {}/{} = {}", base, quote, price),
        None => println!("Unpriceable"),
    }

}