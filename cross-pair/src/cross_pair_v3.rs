use std::collections::{HashMap, HashSet, VecDeque};

type Currency<'a> = &'a str;
type Rate = f64;
type Quote<'a> = (Currency<'a>, Currency<'a>, Rate);
type Graph<'a> = HashMap<Currency<'a>, HashMap<Currency<'a>, Rate>>;



pub fn create_cross_pair_graph<'a>(quotes: &[Quote<'a>]) -> Graph<'a> {
    let mut graph: Graph<'a> = HashMap::new();

    for &(src, dst, rate) in quotes {
        graph
            .entry(src)
            .or_default()
            .insert(dst, rate);

        graph
            .entry(dst)
            .or_default()
            .insert(src, 1.0/rate);
    }

    graph
}

pub fn derive_cross_pair_price<'a>(graph: &Graph<'a>, base: Currency<'a>, quote: Currency<'a>) -> Option<Rate> {

    if base == quote {
        return Some(1.0);
    }

    let mut visited: HashSet<Currency<'a>> = HashSet::new();
    let mut queue: VecDeque<(Currency<'a>, Rate)> = VecDeque::new();

    visited.insert(base);
    queue.push_back((base, 1.0));



    while let Some((currency, accumulated_rate)) = queue.pop_front() {
        //find neighbors
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