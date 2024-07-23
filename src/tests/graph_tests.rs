use std::cmp::min;
use std::mem::size_of;
use std::time::{Instant};
use crate::{graph};
use crate::algorithms::bfs;
use crate::edge_storage::{header_size_in_msize_units};
use crate::graph::{Graph};
use crate::graph::TraverseResult::Continue;
use crate::size::VHandle;
use crate::traits::{EdgeOperator, EdgeStore, Transformer};

#[test]
pub fn graph_init_test() {
    let mut graph = Graph::new_large();
    assert_eq!(graph.vertices.len(), 0);
    assert_eq!(graph.edges.capacity(), 0);

    graph.create_leaf(1);
    graph.create_leaf(2);
    graph.create_leaf(3);

    assert_eq!(graph.vertices.len(), 3);
    assert_eq!(graph.edges.capacity(), (50+ header_size_in_msize_units())*3);

}

#[test]
pub fn graph_basic_test(){
    let mut graph = Graph::new_large();
    let a = graph.create_leaf("a");
    let b = graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_leaf(a, "a_a");
    graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");

    let b_a = graph.create_and_connect_leaf(b, "b_a");
    graph.create_and_connect_leaf(b, "b_b");

   graph.create_and_connect_leaf(b_a, "b_a_a");

    let a_edges = graph.edges.edges(a);
    assert_eq!(a_edges.len(), 3);

    for edge in a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "a_a"),
            1 => assert_eq!(graph.vertices[*edge], "a_b"),
            2 => assert_eq!(graph.vertices[*edge], "a_c"),
            _ => continue,
        }
    }

    let b_edges = graph.edges.edges(b);
    assert_eq!(b_edges.len(), 2);

    for edge in b_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "b_a"),
            1 => assert_eq!(graph.vertices[*edge], "b_b"),
            _ => continue,
        }
    }

    let b_a_a_edges = graph.edges.edges(b_a);
    assert_eq!(b_a_a_edges.len(), 1);

    for edge in b_a_a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "b_a_a"),
            _ => continue,
        }
    }

}

#[test]
pub fn graph_default_capacity_test(){
    let mut graph = Graph::new_large();
    let count = 50;


    for i in 0..count {
        graph.create_leaf(i);
    }

    assert_eq!(graph.vertices.len(), 50);
    assert_eq!(graph.edges.capacity(), (50+ header_size_in_msize_units())*count);
}

#[test]
pub fn graph_with_capacity_test(){
    let mut graph = graph::Graph::with_reserve(10);
    let count = 100;

    for i in 0..count {
        graph.create_leaf(i);
    }

    assert_eq!(graph.edges.capacity(), (10+ header_size_in_msize_units())*count);
}

#[test]
#[should_panic]
pub fn graph_edge_overflow_test(){
    let mut graph = graph::Graph::with_reserve(3);
    let count = 4;
    let a = graph.create_leaf(0);

    for i in 0..count {
        graph.create_and_connect_leaf(a, i+1);
    }
}


#[test]
pub fn graph_mutability_test(){
    let mut graph = graph::Graph::new_large();
    let a = graph.create_leaf("a");
    graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_leaf(a, "a_a");
    graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");


    let edges = graph.edges.edges(a);
    assert_eq!(edges.len(), 3);

    for edge in edges {
        match *edge{
            0 => {
                graph.vertices[*edge] = "a_a_edited";
                graph.vertices[*edge] = "a_a_edited"
            },
            1 => {
                graph.vertices[*edge] = "a_b_edited";
                graph.vertices[*edge] = "a_b_edited"
            },
            2 => {
                graph.vertices[*edge] = "a_c_edited";
                graph.vertices[*edge] = "a_c_edited"
            },
            _ => continue,
        }
    }
}

#[test]
pub fn graph_transform_bench(){
    let mut graph = Graph::new_large();
    let test_size = min(size_of::<VHandle>(), 10000000) as VHandle;

    for i in 0..test_size {
        graph.create_leaf(i);
    }
    let start = Instant::now();
    graph.vertices.transform(|slice| {
        for i in slice{
            *i = *i * 10;
        }
    });
    println!("Time taken: {:?}", start.elapsed());
    for i in 0..test_size {
        assert_eq!(graph.vertices[i], i*10);
    }


}

#[test]
pub fn graph_transform_bench_async(){
    let mut graph = Graph::new_large();
    let test_size = min(size_of::<VHandle>(), 10000000) as VHandle;

    for i in 0..test_size {
        graph.create_leaf(i);
    }
    let start = Instant::now();
    graph.vertices.async_transform(|slice| {
        for i in slice{
            *i = *i * 10;
        }
    });
    println!("Time taken: {:?}", start.elapsed());

    for i in 0..test_size {
        assert_eq!(graph.vertices[i], i*10);
    }


}
#[test]
pub fn graph_disconnect_test(){
    let mut graph = Graph::new_large();
    let a = graph.create_leaf("a");
    graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_leaf(a, "a_a");
    let ab= graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");
    let ad= graph.create_and_connect_leaf(a, "a_d");
    graph.create_and_connect_leaf(a, "a_e");
    let af= graph.create_and_connect_leaf(a, "a_f");
    graph.edges.disconnect(a, af);


    assert_eq!(graph.edges.len(a), 5);
    let edges = graph.edges.edges(a);

    for edge in edges {
        match *edge{
            3 => assert_eq!(graph.vertices[*edge], "a_a"),
            4 => assert_eq!(graph.vertices[*edge], "a_b"),
            5 => assert_eq!(graph.vertices[*edge], "a_c"),
            6 => assert_eq!(graph.vertices[*edge], "a_d"),
            7 => assert_eq!(graph.vertices[*edge], "a_e"),
            _ => continue,
        }
    }

    graph.edges.disconnect(a, ad);
    graph.edges.disconnect(a, ab);

    assert_eq!(graph.edges.len(a), 3);

    let edges = graph.edges.edges(a);
    for edge in edges {
        match *edge{
            3 => assert_eq!(graph.vertices[*edge], "a_a"),
            5 => assert_eq!(graph.vertices[*edge], "a_c"),
            7 => assert_eq!(graph.vertices[*edge], "a_e"),
            _ => continue,
        }
    }

}
#[test]
pub fn graph_bfs_test(){
    let mut graph = Graph::new_large();
    let root = graph.create_leaf("root");
    let a = graph.create_and_connect_leaf(root, "a");
    let b = graph.create_and_connect_leaf(root, "b");
    graph.create_and_connect_leaf(root, "c");

    graph.create_and_connect_leaf(a, "a_a");
    graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");

    let b_a = graph.create_and_connect_leaf(b, "b_a");
    graph.create_and_connect_leaf(b, "b_b");

    graph.create_and_connect_leaf(b_a, "b_a_a");

    let mut snap = vec![
        "b_a_a".to_string(),
        "b_b".to_string(),
        "b_a".to_string(),
        "a_c".to_string(),
        "a_b".to_string(),
        "a_a".to_string(),
        "c".to_string(),
        "b".to_string(),
        "a".to_string(),
        "root".to_string(),
    ];

    bfs(&mut graph.edges, root, graph.vertices.len(), |_edges, handle|{
        assert_eq!(graph.vertices[handle], snap.pop().unwrap());
        Continue
    });
}

#[test]
pub fn graph_static_test(){
    let mut graph = Graph::new();
    let root = graph.create("root", 5);
    let a = graph.create_and_connect(root,"a", 1);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    graph.create_and_connect(root, "b", 0);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    graph.create_and_connect(root,"c", 0);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    let d = graph.create_and_connect(root, "d", 1);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    let e = graph.create_and_connect(root, "e", 1);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);

    graph.create_and_connect(a, "a_a", 0);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    graph.create_and_connect(d, "a_d", 0);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    graph.create_and_connect(e, "a_e", 0);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
    assert_eq!(graph.edges.edge_block_capacity(root), 5);
}