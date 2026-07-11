use std::collections::{HashMap, HashSet, VecDeque};

/*
Time Complexity:

Graph Consurctiion: O(E) //every quote creates two graph edges
BFS Query : O(V+E) //because BFS may visit every currency and inspect evry edge


Space Complexity:
Graph Storage: O(V + E) //because adjacency list stores currencies and edges
BFS Aux Space: O(V) //becuae queue and visited can hold upto all currencies

Total Program Space = O(V+E)


*/

pub fn create_cross_price_graph(quotes: &Vec<(&'static str, &'static str, f64)>) -> Result<HashMap<&'static str, Vec<(&'static str, f64)>>, String>{
    let mut graph: HashMap<&'static str, Vec<(&'static str, f64)>> = HashMap::new();

    for (base, quote, price) in quotes {
        //Edge case 4: invalid zero/negative price
        if *price <= 0.0 {
            return Err(format!("Invalid price for {}/{}: {}", base, quote, price));
        }


        //Direct Quote
        //EUR/USD = 1.10 means 1 EUR = 1.10 USD
        graph
            .entry(base)
            .or_default()
            .push((quote, *price));

        //Reverse Quote
        graph
            .entry(quote)
            .or_default()
            .push((base, 1.0/price));

    }

    Ok(graph)
}


pub fn derive_cross_price(graph: &HashMap<&'static str, Vec<(&'static str, f64)>>, base: &'static str, quote: &'static str) -> Option<f64> {

    //Edge case 5: self pair
    if base == quote {
        return Some(1.0);
    }

    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut queue: VecDeque<(&'static str, f64)> = VecDeque::new();

    //queue starts
    //(cuurrent_currency, accumulated_conversion_rate)

    //starting from EUR:
    //EUR -> EUR = 1.0
    visited.insert(base);
    queue.push_back((base, 1.0));

    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        //find neighbors
        if let Some(neighbors) =  graph.get(currency) {
            for (neighbor, rate) in neighbors {
                if visited.insert(neighbor) {

                    let new_rate = accumulated_rate * rate;
                    if *neighbor == quote {
                        return Some(new_rate);
                    }
                    
                    queue.push_back((neighbor, new_rate));
                }
            }
        }
    }
    //Edge case 1: no path exists
    None
}



pub fn create_cross_price_graph_1<'a>(quotes: &Vec<(&'a str, &'a str, f64)>) -> Result<HashMap<&'a str, Vec<(&'a str, f64)>>, String> {

    let mut graph:HashMap<&'a str, Vec<(&'a str, f64)>> = HashMap::new();

    /*
        Edge case to handle in this block:
        - if price given for a pair is negative or 0 then send error back and stop processing
     */

    for &(src, dst, price) in quotes {

        //handling when price <= 0

        if price <= 0.0 {
            return Err(format!("Invalid price found, not proceeding with graph creation for {src}/{dst} = {price}"));
        }

        graph
            .entry(src)
            .or_default()
            .push((dst, price));

        graph
            .entry(dst)
            .or_default()
            .push((src, 1.0/price));

    }
    Ok(graph) 
}

pub fn derive_cross_price_1<'a>(graph: &HashMap<&'a str, Vec<(&'a str, f64)>>, base: &'a str, quote: &'a str) -> Option<f64> {

    //Edge case if base == quote
    if base == quote {
        return Some(1.0);
    }

    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut queue: VecDeque<(&'a str, f64)> = VecDeque::new();

    visited.insert(base);
    queue.push_back((base, 1.0));

    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(currency) {
            for (neighbor, rate) in neighbors {
                if visited.insert(neighbor) {
                    let new_rate =  accumulated_rate * rate;

                    if quote == *neighbor {
                        return Some(new_rate);
                    }
                    queue.push_back((neighbor, new_rate));
                }
            }
        }
    }
    None
}