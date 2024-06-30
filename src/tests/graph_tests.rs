use std::{thread, time};
use std::time::{Instant, SystemTime};
use crate::graph;
use crate::graph::header_size_to_elements;
use crate::traits::Transform;

#[test]
pub fn graph_init_test() {
    let mut graph = graph::Graph::new();
    assert_eq!(graph.vertices.len(), 0);
    assert_eq!(graph.edges.capacity(), 0);

    graph.create(1);
    graph.create(2);
    graph.create(3);

    assert_eq!(graph.vertices.len(), 3);
    assert_eq!(graph.edges.capacity(), (50+ header_size_to_elements())*3);

}

#[test]
pub fn graph_basic_test(){
    let mut graph = graph::Graph::new();
    let a = graph.create("a");
    let b = graph.create("b");
    graph.create("c");

    graph.create_and_connect(a, "a_a");
    graph.create_and_connect(a, "a_b");
    graph.create_and_connect(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a");
    graph.create_and_connect(b, "b_b");

   graph.create_and_connect(b_a, "b_a_a");

    let a_edges_result = graph.edges.edges(a);
    assert_eq!(a_edges_result.is_err(), false);

    let a_edges = a_edges_result.ok().unwrap();
    assert_eq!(a_edges.len(), 3);

    for edge in a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "a_a"),
            1 => assert_eq!(graph.vertices[*edge], "a_b"),
            2 => assert_eq!(graph.vertices[*edge], "a_c"),
            _ => continue,
        }
    }

    let b_edges_result = graph.edges.edges(b);
    assert_eq!(b_edges_result.is_err(), false);

    let b_edges = b_edges_result.ok().unwrap();
    assert_eq!(b_edges.len(), 2);

    for edge in b_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "b_a"),
            1 => assert_eq!(graph.vertices[*edge], "b_b"),
            _ => continue,
        }
    }

    let b_a_a_edges_result = graph.edges.edges(b_a);
    assert_eq!(b_a_a_edges_result.is_err(), false);

    let b_a_a_edges = b_a_a_edges_result.ok().unwrap();
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
    let mut graph = graph::Graph::new();
    let count = 50;


    for i in 0..count {
        graph.create(i);
    }

    assert_eq!(graph.vertices.len(), 50);
    assert_eq!(graph.edges.capacity(), (50+ header_size_to_elements())*count);
}

#[test]
pub fn graph_with_capacity_test(){
    let mut graph = graph::Graph::with_capacity(10);
    let count = 100;

    for i in 0..count {
        graph.create(i);
    }

    assert_eq!(graph.edges.capacity(), (10+ header_size_to_elements())*count);
}

#[test]
#[should_panic]
pub fn graph_edge_overflow_test(){
    let mut graph = graph::Graph::with_capacity(3);
    let count = 4;
    let a = graph.create(0);

    for i in 0..count {
        graph.create_and_connect(a, i+1);
    }
}


#[test]
pub fn graph_mutability_test(){
    let mut graph = graph::Graph::new();
    let a = graph.create("a");
    graph.create("b");
    graph.create("c");

    graph.create_and_connect(a, "a_a");
    graph.create_and_connect(a, "a_b");
    graph.create_and_connect(a, "a_c");

    let result = graph.edges.edges(a);
    assert_eq!(result.is_err(), false);

    let edges = result.ok().unwrap();
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
pub fn graph_transform_test(){
    let mut graph = graph::Graph::new();
    let test_size = 10000000;

    for i in 0..test_size {
        graph.create(i);
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
pub fn graph_transform_test_async(){
    let mut graph = graph::Graph::new();
    let test_size = 10000000;

    for i in 0..test_size {
        graph.create(i);
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
    let mut graph = graph::Graph::new();
    let a = graph.create("a");
    graph.create("b");
    graph.create("c");

    graph.create_and_connect(a, "a_a");
    let ab= graph.create_and_connect(a, "a_b");
    graph.create_and_connect(a, "a_c");
    let ad= graph.create_and_connect(a, "a_d");
    graph.create_and_connect(a, "a_e");
    let af= graph.create_and_connect(a, "a_f");
    graph.edges.disconnect(a, af);


    assert_eq!(graph.edges.len(a), 5);

    match graph.edges.edges(a){
        Ok(edges) => {
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
        },
        Err(_) => {
            panic!("Vertex not found!");
        }
    }

    graph.edges.disconnect(a, ad);
    graph.edges.disconnect(a, ab);

    assert_eq!(graph.edges.len(a), 3);

    match graph.edges.edges(a){
        Ok(edges) => {
            for edge in edges {
                match *edge{
                    3 => assert_eq!(graph.vertices[*edge], "a_a"),
                    5 => assert_eq!(graph.vertices[*edge], "a_c"),
                    7 => assert_eq!(graph.vertices[*edge], "a_e"),
                    _ => continue,
                }
            }
        },
        Err(_) => {
            panic!("Vertex not found!");
        }
    }

}