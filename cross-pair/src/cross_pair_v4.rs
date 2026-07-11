use std::collections::{HashMap, HashSet, VecDeque};



pub fn create_cross_price_graph<'a>(quotes: &Vec<(&'a str, &'a str, f64)>) -> HashMap<&'a str, Vec<(&'a str, f64)>> {
    let mut graph: HashMap<&'a str, Vec<(&'a str, f64)>> = HashMap::new();

    for &(src, dst, rate) in quotes {
        if rate <= 0.0 {
            println!("Error processing quote {}/{} = {} due to invalid pricing", src, dst, rate);
            continue;
        }

        graph
            .entry(src)
            .or_default()
            .push((dst, rate));

        graph
            .entry(dst) 
            .or_default()
            .push((src, 1.0/rate));
    }
    graph
}

pub fn derive_cross_price_and_path<'a>(graph: &HashMap<&'a str, Vec<(&'a str, f64)>>, base: &'a str, quote: &'a str) -> Option<(f64, Vec<(&'a str, &'a str)>)> {

    if base == quote {
        return Some((1.0, vec![]));
    }

    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut queue: VecDeque<(&'a str, f64)> = VecDeque::new();

    /*
        child -> parent
        Example:
        USD -> EUR
        JPY -> USD
        AUD -> JPY
     */

    let mut parent: HashMap<&'a str, &'a str> = HashMap::new();

    visited.insert(base);
    queue.push_back((base, 1.0));

    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(currency) {
            for &(neighbor, rate) in neighbors {
                if visited.insert(neighbor) {
                    //now we gotta save the child -> parent
                    parent.insert(neighbor, currency);
                    let new_rate = accumulated_rate * rate;

                    if neighbor == quote {
                        //reconstruct the path
                        //save as edges + reverse
                        let mut path: Vec<(&'a str, &'a str)> = Vec::new();

                        let mut current = quote;

                        while current != base {
                            let previous = parent.get(current)?;

                            path.push((previous, current));

                            current = previous;
                        }
                        path.reverse();

                        return Some((new_rate, path));
                    }

                    queue.push_back((neighbor, new_rate));

                }
            } 
        }
    }

    None
}


pub fn update_reverse_index<'a>(reverse_index: &mut HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>>, cross: (&'a str, &'a str), path: Vec<(&'a str, &'a str)>) {
    for edge in path {
        reverse_index
            .entry(edge)
            .or_default()
            .push(cross);
    }
}


pub fn build_reverse_index<'a>(graph: &HashMap<&'a str, Vec<(&'a str, f64)>>, crosses: &Vec<(&'a str, &'a str)>) -> HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>> {

    let mut reverse_index: HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>> = HashMap::new();

    for &cross in crosses {
        let (base, quote) = cross;

        if let Some((_price, path)) = derive_cross_price_and_path(graph, base, quote) {
            update_reverse_index(&mut reverse_index, cross, path);
        }
    }
    reverse_index
}