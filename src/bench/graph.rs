use std::time::Instant;
use firestorm::profile_fn;
use crate::algorithms::bfs;
use crate::graph;
use crate::graph::TraverseResult::Continue;
use crate::size::VHandle;
use crate::traits::EdgeOperator;

#[test]
pub fn graph_disconnect_bench(){
    // prepare data
    let data_size = 200000;
    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size);
    let mut handles = Vec::with_capacity(data_size);
    for i in 0..data_size {
        handles.push(graph.create_and_connect_leaf(root, i+1));
    }

    let start = Instant::now();
    while handles.len() > 0 {
        let handle = handles.pop().unwrap();
        graph.edges.disconnect(root, handle);
    }
    println!("Time taken: {:?}", start.elapsed());
}

#[test]
pub fn graph_disconnect_safe_bench(){
    // prepare data
    let data_size = 20000;
    let mut graph = graph::Graph::with_reserve(data_size);
    let root = graph.create_leaf(0);
    for i in 0..data_size {
        graph.create_and_connect_leaf(root, i+1);
    }

    let start = Instant::now();
    for i in 0..data_size as VHandle {
        graph.edges.disconnect(root, i+1);
    }
    println!("Time taken: {:?}", start.elapsed());
}
#[test]
pub fn bfs_bench_firestorm(){
    if firestorm::enabled() {
        firestorm::bench("./flames/", bfs_bench).unwrap();
    }
}

#[test]
pub fn bfs_bench(){
    // prepare data
    let data_size = 1020;
    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size);
    let mut number_of_nodes = 1;
    for i in 0..data_size {
        let child = graph.create_and_connect(root, i+1, data_size);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_leaf(child, j*data_size);
            number_of_nodes += 1;
        }
    }

    let start = Instant::now();
    let mut counter = 0;
    bfs(&mut graph.edges, root, number_of_nodes, |_edges, vertex|{
        profile_fn!("bfs_transform");
        graph.vertices[vertex] = 0;
        counter += 1;
        return Continue;
    });

    println!("Time taken: {:?}", start.elapsed());
    assert_eq!(counter, number_of_nodes);
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}