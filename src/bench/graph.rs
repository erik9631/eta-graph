use std::time::Instant;
use firestorm::profile_fn;
use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::Resume;
use crate::graph;
use crate::handles::types::VHandle;
use crate::handles::{vh, vh_pack};
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
    while !handles.is_empty() {
        let handle = handles.pop().unwrap();
        graph.edge_storage.disconnect(root, handle);
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
        graph.edge_storage.disconnect(root, i+1);
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
    bfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex, layer|{
        profile_fn!("bfs");
        graph.vertices[vh(*vertex)] = 0;
        counter += 1;
        Resume
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
    dfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex| {
        profile_fn!("dfs");
        graph.vertices[vh(*vertex)] = 0;
        counter += 1;
        Resume
    }, |vertex| {});

    println!("Time taken: {:?}", start.elapsed());
    assert_eq!(counter, number_of_nodes);
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}