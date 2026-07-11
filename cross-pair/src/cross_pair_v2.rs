/*
Improvements from version 1:

1. I will not throw Error if invalid quote comes and will simply skip it

2. I will store neighbors in inner HashMap instead of Vec, so if its a live stream then update will replace the stale quote

*/

use std::collections::{HashMap, VecDeque, HashSet};


pub fn create_cross_price_graph<'a>(quotes: &Vec<(&'a str, &'a str, f64)>) -> HashMap<&'a str, HashMap<&'a str, f64>> {
    let mut graph: HashMap<&'a str, HashMap<&'a str, f64>> = HashMap::new();

    for &(src, dst, price) in quotes {
        if price <= 0.0 {
            println!("Skipping invalid quote: {}/{} = {}", src, dst, price);
            continue;
        }

        graph
            .entry(src)
            .or_default()
            .insert(dst, price);

        graph
            .entry(dst)
            .or_default()
            .insert(src, 1.0/price);
    }
    graph
}


pub fn derive_cross_price<'a>(graph: &HashMap<&'a str, HashMap<&'a str, f64>>, base: &'a str, quote: &'a str) -> Option<f64> {

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
                    let new_rate = accumulated_rate * rate;
                    if *neighbor == quote {
                        return Some(new_rate);
                    }

                    queue.push_back((neighbor, new_rate));
                }
            }
        }
    }

    None
    
}