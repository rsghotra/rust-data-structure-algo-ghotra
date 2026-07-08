use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;

/*
Time complexity: O(V + 2E)

Space complexity: Graph(Adjacency List): O(V+E) + Queue(O(V)) + Visited(O(V)) => O(V + E)

*/

pub fn bfs_unweighted(graph: &HashMap<&'static str, Vec<&'static str>>, start: &'static str) {

    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        print!("{}, ", node);

       if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
       }
    }
    println!();
}

pub fn bfs_weighted(graph: &HashMap<&'static str, Vec<(&'static str, f64)>>, start: &'static str) {

    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        print!("{}, ", node);

        //find neighbors
        if let Some(neighbors) = graph.get(node) {
            //traverse the neighbors
            for (neighbor, _) in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    println!();
}
