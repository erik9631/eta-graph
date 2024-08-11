use std::cmp::{min};
use std::mem::size_of;
use std::time::{Instant};
use crate::{graph};
use crate::edge_storage::{HEADER_SIZE};
use crate::graph::{Graph};
use crate::handles::types::{VHandle, Weight};
use crate::handles::{vh, wgt};
use crate::traits::{GraphOperate, EdgeStore, StoreVertex};
use crate::weighted_graph::WeightedGraph;

#[test]
pub fn graph_init_test() {
    let mut graph = Graph::new_large();
    assert_eq!(graph.vertices.len(), 0);
    assert_eq!(graph.edge_storage.capacity(), 0);

    graph.create_leaf(1);
    graph.create_leaf(2);
    graph.create_leaf(3);

    assert_eq!(graph.vertices.len(), 3);
    assert_eq!(graph.edge_storage.capacity(), (50+ HEADER_SIZE)*3);

}

#[test]
pub fn graph_basic_test(){
    let mut graph = Graph::new_large();
    let a = graph.create_leaf("a");
    let b = graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_0(a, "a_a");
    graph.create_and_connect_0(a, "a_b");
    graph.create_and_connect_0(a, "a_c");

    let b_a = graph.create_and_connect_0(b, "b_a");
    graph.create_and_connect_0(b, "b_b");

   graph.create_and_connect_0(b_a, "b_a_a");

    let a_edges = graph.edge_storage.edges(a);
    assert_eq!(a_edges.len(), 3);

    for edge in a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[vh(*edge)], "a_a"),
            1 => assert_eq!(graph.vertices[vh(*edge)], "a_b"),
            2 => assert_eq!(graph.vertices[vh(*edge)], "a_c"),
            _ => continue,
        }
    }

    let b_edges = graph.edge_storage.edges(b);
    assert_eq!(b_edges.len(), 2);

    for edge in b_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[vh(*edge)], "b_a"),
            1 => assert_eq!(graph.vertices[vh(*edge)], "b_b"),
            _ => continue,
        }
    }

    let b_a_a_edges = graph.edge_storage.edges(b_a);
    assert_eq!(b_a_a_edges.len(), 1);

    for edge in b_a_a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[vh(*edge)], "b_a_a"),
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
    assert_eq!(graph.edge_storage.capacity(), (50+ HEADER_SIZE)*count);
}

#[test]
pub fn graph_with_capacity_test(){
    let mut graph = graph::Graph::with_reserve(10);
    let count = 100;

    for i in 0..count {
        graph.create_leaf(i);
    }

    assert_eq!(graph.edge_storage.capacity(), (10+ HEADER_SIZE)*count);
}

#[test]
#[should_panic]
pub fn graph_edge_overflow_test(){
    let mut graph = graph::Graph::with_reserve(3);
    let count = 4;
    let a = graph.create_leaf(0);

    for i in 0..count {
        graph.create_and_connect_0(a, i+1);
    }
}


#[test]
pub fn graph_mutability_test(){
    let mut graph = graph::Graph::new_large();
    let a = graph.create_leaf("a");
    graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_0(a, "a_a");
    graph.create_and_connect_0(a, "a_b");
    graph.create_and_connect_0(a, "a_c");


    let edges = graph.edge_storage.edges(a);
    assert_eq!(edges.len(), 3);

    for edge in edges {
        match *edge{
            0 => {
                graph.vertices[vh(*edge)] = "a_a_edited";
                graph.vertices[vh(*edge)] = "a_a_edited"
            },
            1 => {
                graph.vertices[vh(*edge)] = "a_b_edited";
                graph.vertices[vh(*edge)] = "a_b_edited"
            },
            2 => {
                graph.vertices[vh(*edge)] = "a_c_edited";
                graph.vertices[vh(*edge)] = "a_c_edited"
            },
            _ => continue,
        }
    }
}

#[test]
pub fn graph_vertices_iter_test(){
    let mut graph = Graph::new_large();
    let test_size = min(size_of::<VHandle>(), 10000000) as VHandle;

    for i in 0..test_size {
        graph.create_leaf(i);
    }
    let start = Instant::now();
    for vertice in graph.vertices.iter_mut(){
        *vertice *= 10;
    }
    println!("Time taken: {:?}", start.elapsed());
    for i in 0..test_size {
        assert_eq!(graph.vertices[i], i*10);
    }

}

#[test]
pub fn graph_edges_iter_test(){
    let mut graph = Graph::new();
    let test_size = 100;
    let mut last = graph.create(0, 5);
    let mut vals = 0;
    for i in 0..test_size {
        graph.create_and_connect_0(last, vals+1);
        graph.create_and_connect_0(last, vals+2);
        graph.create_and_connect_0(last, vals+3);
        graph.create_and_connect_0(last, vals+4);
       last =  graph.create_and_connect(last, vals+5, 5);
        vals += 5;
    }
    let counter = 1;

    let mut iter = graph.edge_storage.iter_mut();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    *a = 10;
    *b = 20;

    println!("{} {}", *a, *b);

    // for (index, item) in graph.edge_storage.iter().enumerate_as_index(){
    //     assert_eq!(*item as usize, counter);
    //     counter += 1;
    // }
}
//
// #[test]
// pub fn graph_transform_bench_async(){
//     let mut graph = Graph::new_large();
//     let test_size = min(size_of::<VHandle>(), 10000000) as VHandle;
//
//     for i in 0..test_size {
//         graph.create_leaf(i);
//     }
//     let start = Instant::now();
//     graph.vertices.async_transform(|slice| {
//         for i in slice{
//             *i = *i * 10;
//         }
//     });
//     println!("Time taken: {:?}", start.elapsed());
//
//     for i in 0..test_size {
//         assert_eq!(graph.vertices[i], i*10);
//     }
//
//
// }
#[test]
pub fn graph_disconnect_test(){
    let mut graph = Graph::new_large();
    let a = graph.create_leaf("a");
    graph.create_leaf("b");
    graph.create_leaf("c");

    graph.create_and_connect_0(a, "a_a");
    let ab= graph.create_and_connect_0(a, "a_b");
    graph.create_and_connect_0(a, "a_c");
    let ad= graph.create_and_connect_0(a, "a_d");
    graph.create_and_connect_0(a, "a_e");
    let af= graph.create_and_connect_0(a, "a_f");
    graph.edge_storage.disconnect(a, af);


    assert_eq!(graph.edge_storage.len(a), 5);
    let edges = graph.edge_storage.edges(a);

    for edge in edges {
        match *edge{
            3 => assert_eq!(graph.vertices[vh(*edge)], "a_a"),
            4 => assert_eq!(graph.vertices[vh(*edge)], "a_b"),
            5 => assert_eq!(graph.vertices[vh(*edge)], "a_c"),
            6 => assert_eq!(graph.vertices[vh(*edge)], "a_d"),
            7 => assert_eq!(graph.vertices[vh(*edge)], "a_e"),
            _ => continue,
        }
    }

    graph.edge_storage.disconnect(a, ad);
    graph.edge_storage.disconnect(a, ab);

    assert_eq!(graph.edge_storage.len(a), 3);

    let edges = graph.edge_storage.edges(a);
    for edge in edges {
        match *edge{
            3 => assert_eq!(graph.vertices[vh(*edge)], "a_a"),
            5 => assert_eq!(graph.vertices[vh(*edge)], "a_c"),
            7 => assert_eq!(graph.vertices[vh(*edge)], "a_e"),
            _ => continue,
        }
    }

}

#[test]
pub fn graph_static_test(){
    let mut graph = Graph::new();
    let root = graph.create("root", 5);
    let a = graph.create_and_connect(root,"a", 1);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    graph.create_and_connect(root, "b", 0);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    graph.create_and_connect(root,"c", 0);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    let d = graph.create_and_connect(root, "d", 1);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    let e = graph.create_and_connect(root, "e", 1);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);

    graph.create_and_connect(a, "a_a", 0);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    graph.create_and_connect(d, "a_d", 0);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    graph.create_and_connect(e, "a_e", 0);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
    assert_eq!(graph.edge_storage.edge_block_capacity(root), 5);
}


#[test]
pub fn graph_weight_test(){
    let mut wgraph = WeightedGraph::new();
    let root = wgraph.graph.create("root", 5);
    wgraph.create_and_connect_weighted(root, "a", 5, 0);
    wgraph.create_and_connect_weighted(root, "b", 7, 0);
    wgraph.create_and_connect_weighted(root, "c", 1052, 0);
    wgraph.create_and_connect_weighted(root, "d", Weight::MAX, 0);
    wgraph.create_and_connect_weighted(root, "e", -Weight::MAX, 0);

    assert_eq!(wgraph.graph.edge_storage.len(root), 5);

    for edge in wgraph.graph.edge_storage.edges(root){
        match *edge{
            0 => {
                assert_eq!(wgt(*edge), 5);
                assert_eq!(vh(*edge), 1);
            },
            1 => {
                assert_eq!(wgt(*edge), 7);
                assert_eq!(vh(*edge), 2);
            },
            2 => {
                assert_eq!(wgt(*edge), 1052);
                assert_eq!(vh(*edge), 3);
            },
            3 => {
                assert_eq!(wgt(*edge), Weight::MAX);
                assert_eq!(vh(*edge), 4);
            },
            4 => {
                assert_eq!(wgt(*edge), -Weight::MAX);
                assert_eq!(vh(*edge), 5);
            },
            _ => continue,
        }
    }
}