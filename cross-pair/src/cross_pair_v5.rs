use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};




pub fn create_cross_price_graph<'a>(quotes: &Vec<(&'a str, &'a str, f64)>) -> HashMap<&'a str, Vec<(&'a str, f64)>> {
    let mut graph: HashMap<&'a str, Vec<(&'a str, f64)>> = HashMap::new();


    for &(src, dst, rate) in quotes {
        if rate <= 0.0 {
            println!("Invalid price detected for pair {}/{} = {}", src, dst, rate);
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

pub fn derive_cross_price_and_path<'a>(graph: &HashMap<&'a str, Vec<(&'a str, f64)>>, base: &'a str, quote: &'a str)
    -> Option<(f64, Vec<(&'a str, &'a str)>)> {

    if base == quote {
        return Some((1.0, vec![]));
    }

    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut queue: VecDeque<(&'a str, f64)> = VecDeque::new();

    /*
        child -> parent
        USD -> EUR
        JPY -> USD
     */
    let mut parent: HashMap<&'a str, &'a str> = HashMap::new();


    visited.insert(base);
    queue.push_back((base, 1.0));

    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(currency) {
            for &(neighbor, rate) in neighbors {
                if visited.insert(neighbor) {
                    let new_rate = accumulated_rate * rate;
                    //save child -> parent
                    parent.insert(neighbor, currency);

                    if neighbor == quote {
                        let mut path: Vec<(&'a str, &'a str)> = Vec::new();

                        let mut current = neighbor;
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

pub fn update_reverse_index<'a>(reverse_index: &mut HashMap<(&'a str, &'a str), Vec<(&'a str, &'a str)>>, base: &'a str, quote: &'a str, path: &Vec<(&'a str, &'a str)>) {
    for &edge in path {
        reverse_index
            .entry(edge)
            .or_default()
            .push((base, quote));
    }
}

/*
Practice 2

*/

pub fn derive_cross_price_and_path_1<'a>(graph: &HashMap<&'a str, Vec<(&'a str, f64)>>, base: &'a str, quote: &'a str) ->
    Option<(f64, Vec<(&'a str, &'a str)>)> {

    let mut graph: HashMap<&'a str, Vec<(&'a str, f64)>> = HashMap::new();

    if base == quote {
        return Some((1.0, vec![]));
    }

    let mut visited:HashSet<&'a str> = HashSet::new();
    let mut queue:VecDeque<(&'a str, f64)> = VecDeque::new();
    let mut parent: HashMap<&'a str, &'a str> = HashMap::new();

    visited.insert(base);
    queue.push_back((base, 1.0));

    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(currency) {
            for &(neighbor, rate) in neighbors {
                if visited.insert(neighbor) {
                    parent.insert(neighbor, currency);
                    let new_rate = accumulated_rate * rate;

                    if neighbor == quote {
                        let mut current = quote;
                        let mut path: Vec<(&'a str, &'a str)> = Vec::new();
                        
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