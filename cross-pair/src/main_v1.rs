mod cross_pair_v1;

fn main() {
     
    /*
        V1
        Edge Cases:
        1. Cycles
        2. If no path found
        3. Self Pair
        4. If 0 
        5. Pair given one way
        6. Stale prices
        7. Reverse dependency index for selctive update
     */

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

    /*
        Version 1
     */

    println!("VERSION 1 BEGINS -----------");

    let graph = match cross_pair_v1::create_cross_price_graph(&quotes) {
        Ok(g) => g,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    println!("Graph created by version 1: {:?}", graph);


    match cross_pair_v1::derive_cross_price(&graph, "EUR", "JPY") {
        Some(price) => println!("EUR/JPY = {}", price),
        None => println!("Unpriceable"),
    }
    println!("VERSION 1 END -----------");


    /*
    
    
    */

    let graph = match cross_pair_v1::create_cross_price_graph_1(&quotes) {
        Ok(g) => g,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    println!("Graph created by version 1.1: {:?}", graph);
    let base = "EUR";
    let quote = "JPY"; 

    match cross_pair_v1::derive_cross_price_1(&graph, base, quote) {
        Some(price) => println!("The cross price for {}/{} = {}", base, quote, price),
        None => println!("Unpriceable"),
    }


}
