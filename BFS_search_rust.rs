// We have a graph with N nodes, and each node is connected to some other nodes.
// Given this graph as an adjacency list, we want to print all the nodes connected to a given starting node using Breadth-First Search (BFS).
// Suppose: The graph is given as     
//    (0) -- (1) -- (2)
//     |      
//    (3) -- (4)
// Adjacency list is given by:
// 0 → [1, 3]
//1 → [0, 2]
//2 → [1]
//3 → [0, 4]
//4 → [3]
//---- let's start in rust --------

use std::collections::{HashMap, VecDeque, HashSet};

fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) {
    let mut queue = VecDeque::new();  // Queue for BFS
    let mut visited = HashSet::new(); // Track visited nodes

    queue.push_back(start);
    visited.insert(start);

    print!("Visited nodes: ");

    while let Some(node) = queue.pop_front() {
        print!("{} ", node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
    }

    println!("End of this function");
}

fn main() {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // Define the graph as an adjacency list
    graph.insert(0, vec![1, 3]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![1]);
    graph.insert(3, vec![0, 4]);
    graph.insert(4, vec![3]);

    let start_node = 0;
    bfs(&graph, start_node);

    println!("End of this function");

}
