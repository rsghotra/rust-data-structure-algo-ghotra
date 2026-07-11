use std::collections::HashMap;
mod cross_pair_v5;

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

    let graph = cross_pair_v5::create_cross_price_graph(&quotes);

    let mut reverse_index: HashMap<(&str, &str), Vec<(&str, &str)>> = HashMap::new();

    let base = "USD";
    let quote = "AUD";

    if let Some((price, path)) = cross_pair_v5::derive_cross_price_and_path(&graph, base, quote) {
        println!("Price for {}/{} = {}", base, quote, price);
        println!("Path traversed: {:?}", path);
        cross_pair_v5::update_reverse_index(&mut reverse_index, base, quote, &path);
    }

    println!("Reverse index: {:?}", reverse_index);

    
}