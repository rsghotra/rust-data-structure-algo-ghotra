use std::collections::HashMap;

pub fn get_unweighted_undirected_edge_list() -> Vec<(&'static str, &'static str)> {

    vec![
        ("EUR", "USD"),
        ("EUR", "GBP"),
        ("USD", "GBP"),
        ("USD", "JPY"),
        ("JPY", "AUD"),
    ]
}

pub fn get_unweighted_directed_edge_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("EUR", "USD"),
        ("EUR", "GBP"),
        ("USD", "JPY"),
        ("GBP", "JPY"),
        ("JPY", "AUD"),
    ]
}

pub fn get_weighted_undirected_edge_list() -> Vec<(&'static str, &'static str, f64)> {
    vec![
        ("EUR", "USD", 0.91),
        ("EUR", "GBP", 0.86),
        ("USD", "GBP", 0.78),
        ("USD", "JPY", 145.0),
        ("JPY", "AUD", 0.61),
    ]
}

pub fn get_weighted_directed_edge_list() -> Vec<(&'static str, &'static str, f64)> {
     vec![
        ("EUR", "USD", 1.10),
        ("EUR", "GBP", 0.86),
        ("USD", "JPY", 145.0),
        ("GBP", "JPY", 150.0),
        ("JPY", "AUD", 0.61),
    ]
}

pub fn create_weighted_undirected_graph(edges: Vec<(&'static str, &'static str, f64)>) -> HashMap<&'static str, Vec<(&'static str, f64)>>{

    let mut graph: HashMap<&'static str, Vec<(&'static str, f64)>> = HashMap::new();

    for(src, dst, weight) in edges {
        graph
            .entry(src)
            .or_default()
            .push((dst, weight));

        graph
            .entry(dst)
            .or_default()
            .push((src, 1.0/weight));
    }

    graph

}

pub fn create_unweighted_undirected_graph(edges: Vec<(&'static str, &'static str)>) -> HashMap<&'static str, Vec<&'static str>> {

    let mut graph: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    for (src, dst) in edges {
        graph
            .entry(src)
            .or_default()
            .push(dst);

        graph
            .entry(dst)
            .or_default()
            .push(src);
    }

    graph

}

pub fn create_unweighted_directed_graph(edges: Vec<(&'static str, &'static str)>) -> HashMap<&'static str, Vec<&'static str>> {
    let mut graph: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    for (src, dst) in edges {
        graph
            .entry(src)
            .or_default()
            .push(dst);
    }

    graph
}

pub fn create_weighted_directed_graph(edges: Vec<(&'static str, &'static str, f64)>) -> HashMap<&'static str, Vec<(&'static str, f64)>> {
    let mut graph: HashMap<&'static str, Vec<(&'static str, f64)>> = HashMap::new();

    for (src, dst, weight) in edges {
        graph
            .entry(src)
            .or_default()
            .push((dst, weight));
    }

    graph 
}