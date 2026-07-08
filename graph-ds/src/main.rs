mod bfs;
mod graph;

/*
Types of Graphs:
1. Undirected + Unweighted
2. Directed + Unweighted
3. Undirected + Weighted
4. Directed + Weighted

Graphs Representation:

1. Edge List: For reading graph input and converting to Adj List/Matrix, Kruskal MST, Bellman-Ford algo
2. Adjacency List: For traversal, Shortest Path, Has Path types algo which are dependent upon frequent neighbor access; Ex: BFS, DFS, Dijkastra's shortest path
3. Adjacency Matrix: For dense graph and where edge exists operations are frequent; Floydd-Warshall, Constant time edge existence checks

*/

fn main() {
    println!("Welcome to the world of Graph");

    let unweighted_undirected_edges = graph::get_unweighted_undirected_edge_list();
    let unweighted_undirected_graph = graph::create_unweighted_undirected_graph(unweighted_undirected_edges);
    println!("Unweighted + Undirected Graph: {:?}", unweighted_undirected_graph);
    println!("BFS on Unweighted + Undirected Graph: ");
    bfs::bfs_unweighted(&unweighted_undirected_graph, "EUR");
    println!("=========");

    let weighted_undirected_edges = graph::get_weighted_undirected_edge_list();
    let weighted_undirected_graph = graph::create_weighted_undirected_graph(weighted_undirected_edges);
    println!("Weighted + Undirected Graph: {:?}", weighted_undirected_graph);
    println!("BFS on Weighted + Undirected Graph: ");
    bfs::bfs_weighted(&weighted_undirected_graph, "EUR");
    println!("##########");

    let unweighted_directed_edges = graph::get_unweighted_directed_edge_list();
    let unweighted_directed_graph = graph::create_unweighted_directed_graph(unweighted_directed_edges);
    println!("Unweighted + Directed Graph: {:?}", unweighted_directed_graph);
    println!("BFS on Unweighted + Directed Graph: ");
    bfs::bfs_unweighted(&unweighted_directed_graph, "EUR");
    println!("*********");

    let weighted_directed_edges = graph::get_weighted_directed_edge_list();
    let weighted_directed_graph = graph::create_weighted_directed_graph(weighted_directed_edges);
    println!("Weighted + Directed Graph: {:?}", weighted_directed_graph);
    bfs::bfs_weighted(&weighted_directed_graph, "EUR");

}
