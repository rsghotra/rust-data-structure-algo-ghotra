mod cross_pair_v4;

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

    let crosses = vec![
        ("EUR", "JPY"),
        ("EUR", "AUD"),
        ("USD", "AUD"),
        ("EUR", "CHF"),
    ];

    let graph = cross_pair_v4::create_cross_price_graph(&quotes);

    let base = "EUR";
    let quote = "JPY";

    if let Some((price, path)) = cross_pair_v4::derive_cross_price_and_path(&graph, base, quote) {
        println!("{}/{} = {:?}", base, quote, price);
        println!("Path used = {:?}", path);
    }

    let reverse_index = cross_pair_v4::build_reverse_index(&graph, &crosses);

     println!("{:?}", reverse_index);

}