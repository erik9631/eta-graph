use std::io::{Read, stdin};
use std::time::Instant;
use crate::graph;
use crate::graph::MSize;

#[test]
pub fn graph_disconnect_bench(){
    // prepare data
    let data_size = 20000;
    let mut graph = graph::Graph::with_reserve(data_size);
    let root = graph.create_leaf(0);
    for i in 0..data_size {
        graph.create_and_connect_leaf(root, i+1);
    }

    let start = Instant::now();
    for i in 0..data_size as MSize {
        graph.edges.disconnect(root, i+1);
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
    for i in 0..data_size as MSize{
        graph.edges.disconnect(root, i+1);
    }
    println!("Time taken: {:?}", start.elapsed());
}

#[test]
pub fn bfs_bench(){
    // prepare data
    let data_size = 1020;
    let mut graph = graph::Graph::with_reserve(data_size);
    let root = graph.create_leaf(0);
    let mut number_of_nodes = 1;
    for i in 0..data_size {
        let child = graph.create_and_connect_leaf(root, i+1);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_leaf(child, (j*data_size));
            number_of_nodes += 1;
        }
    }

    let start = Instant::now();
    let vec = graph.bfs(root);
    let mut counter = 0;

    for i in vec {
        graph.vertices[i] = 0;
        counter += 1;
    }
    println!("Time taken: {:?}", start.elapsed());

    assert_eq!(counter, number_of_nodes);
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}

#[test]
pub fn bfs_transform_bench(){
    // prepare data
    let data_size = 1020;
    let mut graph = graph::Graph::with_reserve(data_size);
    let root = graph.create_leaf(0);
    let mut number_of_nodes = 1;
    for i in 0..data_size {
        let child = graph.create_and_connect_leaf(root, i+1);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_leaf(child, (j*data_size));
            number_of_nodes += 1;
        }
    }

    let start = Instant::now();
    let mut counter = 0;
    graph.bfs_transform(root, |graph, vertex|{
        graph.vertices[vertex] = 0;
        counter += 1;
    });
    println!("Time taken: {:?}", start.elapsed());

    assert_eq!(counter, number_of_nodes);
    println!("len: {:?}", graph.edges.edges.capacity());
    println!("Counter: {:?}", counter);
    println!("Number of nodes: {:?}", number_of_nodes);
}