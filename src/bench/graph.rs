use std::time::Instant;
use firestorm::profile_fn;
use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::Resume;
use crate::graph;
use crate::handles::types::VHandle;
use crate::handles::vh;
use crate::traits::GraphOperate;

#[test]
pub fn graph_disconnect_bench(){
    // prepare data
    let data_size = 64000;
    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size);
    let mut handles = Vec::with_capacity(data_size as usize);
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
        firestorm::bench("./flames/bfs", bfs_bench).unwrap();
    }
}

#[test]
pub fn dfs_bench_firestorm(){
    if firestorm::enabled() {
        firestorm::bench("./flames/dfs", dfs_bench).unwrap();
    }
}


#[test]
pub fn bfs_bench(){
    // prepare data
    let data_size = 2000;
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
    bfs(&mut graph.edges, root, number_of_nodes, |_edges, vertex, layer|{
        profile_fn!("bfs_transform");
        graph.vertices[vertex] = 0;
        counter += 1;
        return Resume;
    });

    println!("Time taken: {:?}", start.elapsed());
    assert_eq!(counter, number_of_nodes);
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}


#[test]
pub fn dfs_bench(){
    // prepare data
    let data_size = 2000;
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
    dfs(&mut graph.edges, root, number_of_nodes, |vertex|{
        profile_fn!("dfs_transform");
        graph.vertices[vh(*vertex)] = 0;
        counter += 1;
        return Resume;
    }, |vertex|{});

    println!("Time taken: {:?}", start.elapsed());
    assert_eq!(counter, number_of_nodes);
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}