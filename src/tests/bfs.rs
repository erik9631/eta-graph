use crate::algorithms::general::bfs;
use crate::algorithms::general::ControlFlow::Resume;
use crate::graph::Graph;
use crate::handles::{vh, vh_pack};
use crate::handles::types::Weight;
use crate::traits::EdgeConnect;

#[test]
pub fn graph_bfs_test(){
    let mut graph = Graph::new();
    let root = graph.create("root", 3);
    let a = graph.create_and_connect(root, "a", 3);
    let b = graph.create_and_connect(root, "b", 2);
    graph.create_and_connect_0(root, "c");

    graph.create_and_connect_0(a, "a_a");
    graph.create_and_connect_0(a, "a_b");
    graph.create_and_connect_0(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a", 1);
    graph.create_and_connect_0(b, "b_b");

    graph.create_and_connect_0(b_a, "b_a_a");
    let mut snap: Vec<(String, Weight)> = vec![
        ("b_a_a".to_string(), 3),
        ("b_b".to_string(), 2),
        ("b_a".to_string(), 2),
        ("a_c".to_string(), 2),
        ("a_b".to_string(), 2),
        ("a_a".to_string(), 2),
        ("c".to_string(), 1),
        ("b".to_string(), 1),
        ("a".to_string(), 1),
        ("root".to_string(), 0),
    ];

    bfs(&mut graph.edge_storage, vh_pack(root), graph.vertices.len(), |handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[vh(*handle)], val.0);
        assert_eq!(layer, val.1);
        Resume
    });

    assert_eq!(snap.len(), 0);
}

#[test]
pub fn graph_bfs_test_cyclic(){
    let mut graph = Graph::with_reserve(2);
    let root = graph.create("root", 3);
    let a = graph.create_and_connect(root, "a", 3);
    let b = graph.create_and_connect(root, "b", 2);
    let c = graph.create_and_connect_0(root, "c");
    graph.edge_storage.connect(c, root);

    graph.create_and_connect_0(a, "a_a");
    let a_b = graph.create_and_connect_0(a, "a_b");
    graph.create_and_connect_0(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a", 1);
    graph.create_and_connect_0(b, "b_b");

    let b_a_a = graph.create_and_connect_0(b_a, "b_a_a");
    graph.edge_storage.connect(a_b, b_a_a);
    graph.edge_storage.connect(b_a_a, b_a);

    let mut snap: Vec<(String, Weight)> = vec![
        ("b_a_a".to_string(), 3),
        ("b_b".to_string(), 2),
        ("b_a".to_string(), 2),
        ("a_c".to_string(), 2),
        ("a_b".to_string(), 2),
        ("a_a".to_string(), 2),
        ("c".to_string(), 1),
        ("b".to_string(), 1),
        ("a".to_string(), 1),
        ("root".to_string(), 0),
    ];

    bfs(&mut graph.edge_storage, vh_pack(root), graph.vertices.len(), |handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[vh(*handle)], val.0);
        assert_eq!(layer, val.1);
        Resume
    });
    assert_eq!(snap.len(), 0);
}