use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use crate::algorithms::general::bfs;
use crate::algorithms::general::ControlFlow::Resume;
use crate::algorithms::max_flow::{mark_levels, DinicGraph};
use crate::handles::{eh, eh_pack, wgt};
use crate::handles::types::{Edge, Weight};
use crate::traits::{EdgeStore, WeightedEdgeConnect};
use crate::weighted_graph::WeightedGraph;

#[test]
pub fn level_test(){
    let mut weighted_graph = WeightedGraph::new();
    // Write test for layering
    let a = weighted_graph.graph.create("a", 2);
    let a_a = weighted_graph.create_and_connect_weighted(a, "a_a", 1, 100);
    let a_a_a = weighted_graph.create_and_connect_weighted(a_a, "a_a_a", 1, 20);
    let a_a_x = weighted_graph.create_and_connect_weighted(a_a_a, "a_a_x", 1, 30);

    let a_b = weighted_graph.create_and_connect_weighted(a, "a_b", 20, 3);
    let a_b_a = weighted_graph.create_and_connect_weighted(a_b, "a_b_a", 10, 1);
    let a_b_b = weighted_graph.create_and_connect_weighted(a_b, "a_b_b", 10, 1);
    let a_b_c = weighted_graph.create_and_connect_weighted(a_b, "a_b_c", 10, 1);

    weighted_graph.graph.edge_storage.connect_weighted(a_b_a, a_a_x, 10);
    weighted_graph.graph.edge_storage.connect_weighted(a_b_b, a_a_x, 10);
    weighted_graph.graph.edge_storage.connect_weighted(a_b_c, a_a_x, 10);

    let mut flow_data = Array::new_with_default(weighted_graph.graph.vertices.len(), Weight::MAX);
    let mut queue = Queue::<*mut Edge>::new_pow2_sized(weighted_graph.graph.vertices.len());
    let mut edges_copy = weighted_graph.graph.edge_storage.clone();
    mark_levels(a, a_a_x, &mut edges_copy, &mut queue, &mut flow_data).expect("Sink not found");

    let mut snap = vec![
        ("a_a_x".to_string(), 3),
        ("a_b_c".to_string(), 2),
        ("a_b_b".to_string(), 2),
        ("a_b_a".to_string(), 2),
        ("a_a_a".to_string(), 2),
        ("a_b".to_string(), 1),
        ("a_a".to_string(), 1),
        ("a".to_string(), 0),
    ];

    bfs(&mut edges_copy, eh_pack(a), weighted_graph.graph.vertices.len(), |v_handle, layer|{
        let snap_data = snap.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[eh(*v_handle)], snap_data.0);
        assert_eq!(flow_data[eh(*v_handle) as usize], snap_data.1);
        Resume
    });

    assert_eq!(snap.len(), 0);
}

#[test]
pub fn dinic_test(){
    let mut graph = WeightedGraph::new();
    // Write test for layering
    let a = graph.graph.create("a", 2);
    let a_a = graph.create_and_connect_weighted(a, "a_a", 100, 1);
    let a_a_a = graph.create_and_connect_weighted(a_a, "a_a_a", 20, 1);
    let a_a_x = graph.create_and_connect_weighted(a_a_a, "a_a_x", 30, 0);

    let a_b = graph.create_and_connect_weighted(a, "a_b", 20, 3);
    let a_b_a = graph.create_and_connect_weighted(a_b, "a_b_a", 10, 1);
    let a_b_b = graph.create_and_connect_weighted(a_b, "a_b_b", 10, 1);
    let a_b_c = graph.create_and_connect_weighted(a_b, "a_b_c", 10, 1);

    graph.graph.edge_storage.connect_weighted(a_b_a, a_a_x, 10);
    graph.graph.edge_storage.connect_weighted(a_b_b, a_a_x, 10);
    graph.graph.edge_storage.connect_weighted(a_b_c, a_a_x, 10);

    let dinic_graph = DinicGraph::from(&graph.graph.vertices, &graph.graph.edge_storage, a, a_a_x);

    let mut snap = vec![
        0,10,10,0,10,10,20,20,20,20
    ];

    for edge in dinic_graph.edge_storage.iter(){
        let snap_data = snap.pop().unwrap();
        assert_eq!(snap_data, wgt(*edge));
    }
    assert_eq!(snap.len(), 0);
}