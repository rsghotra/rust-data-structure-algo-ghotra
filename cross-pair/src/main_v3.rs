mod cross_pair_v3;

/*
 1. this version introduce type system for ease of use on top of previous improvements

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

    let graph = cross_pair_v3::create_cross_pair_graph(&quotes);

    let base = "USD";
    let quote = "AUD";

    match cross_pair_v3::derive_cross_pair_price(&graph, base, quote) {
        Some(price) => println!("EUR/JPY = {}", price),
        None => println!("Unpriceable"),
    }
}