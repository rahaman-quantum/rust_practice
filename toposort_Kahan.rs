use std::collections::{HashMap, VecDeque};
// Generic topological sort function
fn topological_sort(graph: &HashMap<&str, Vec<&str>>) -> Option<Vec<&str>> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for (u, neighbors) in graph {
        for &v in neighbors {
            *in_degree.entry(v).or_insert(0) += 1;
        }
    }
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut sorted: Vec<&str> = Vec::new();
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }
    while let Some(u) = queue.pop_front() {
        sorted.push(u);
        if let Some(neighbors) = graph.get(u) {
            for &v in neighbors {
                if let Some(degree) = in_degree.get_mut(v) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }
    }
    if sorted.len() == graph.len() {
        Some(sorted)
    } else {
        None // Cycle detected (not a DAG)
    }
}
fn main() {
    // Bell State Circuit
    let bell_graph: HashMap<&str, Vec<&str>> = [
        ("H", vec!["CX"]),
        ("CX", vec![]),
    ]
    .iter()
    .cloned()
    .collect();
    if let Some(sorted) = topological_sort(&bell_graph) {
        println!("Bell State Topological Sort: {:?}", sorted);
    } else {
        println!("Bell State: Graph has cycles.");
    }
    // GHZ State Circuit
    let ghz_graph: HashMap<&str, Vec<&str>> = [
        ("H", vec!["CX1"]),
        ("CX1", vec!["CX2"]),
        ("CX2", vec![]),
    ]
    .iter()
    .cloned()
    .collect();
    if let Some(sorted) = topological_sort(&ghz_graph) {
        println!("GHZ State Topological Sort: {:?}", sorted);
    } else {
        println!("GHZ State: Graph has cycles.");
    }
    // 3-Qubit QFT Circuit
    let qft_graph: HashMap<&str, Vec<&str>> = [
        ("H0", vec!["R2_01", "SWAP_02"]),
        ("H1", vec!["R2_12"]),
        ("H2", vec!["SWAP_02"]),
        ("R2_01", vec!["R3_02"]),
        ("R3_02", vec!["SWAP_02"]),
        ("R2_12", vec![]),
        ("SWAP_02", vec![]),
    ]
    .iter()
    .cloned()
    .collect();
    if let Some(sorted) = topological_sort(&qft_graph) {
        println!("3-Qubit QFT Topological Sort: {:?}", sorted);
    } else {
        println!("3-Qubit QFT: Graph has cycles.");
    }
}
