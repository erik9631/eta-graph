use std::collections::HashMap;
use std::fmt::format;
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use crate::algorithms::dfs_bfs::bfs;
use crate::algorithms::dfs_bfs::ControlFlow::Resume;
use crate::algorithms::dinic::{mark_levels, DinicGraph};
use crate::handles::{vh, vh_pack, wgt};
use crate::handles::types::{Edge, VHandle, Weight};
use crate::traits::{EdgeStore, StoreVertex, WeightedEdgeConnect};
use crate::utils::print_graph;
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
    let mut queue = Queue::<VHandle>::new_pow2_sized(weighted_graph.graph.vertices.len());
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

    bfs(&mut edges_copy, vh_pack(a), weighted_graph.graph.vertices.len(), |v_handle, layer|{
        let snap_data = snap.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[vh(*v_handle)], snap_data.0);
        assert_eq!(flow_data[vh(*v_handle) as usize], snap_data.1);
        Resume
    });

    assert_eq!(snap.len(), 0);
}

#[test]
/**
       ┌───┐20 ┌─────┐   30
   ┌──►│A_A├──►│A_A_A├──────┐
100│   └───┘   └─────┘      │
   │                        │
  ┌┴┐                    ┌──▼──┐
  │A│                    │A_A_X│
  └┬┘                    └─────┘
   │       10  ┌─────┐ 10 ▲▲▲
 20│      ┌───►│A_B_A├────┘││
   │      │    └─────┘     ││
   │      │                ││
   │    ┌─┴─┐10┌─────┐  10 ││
   └───►│A_B├─►│A_B_B├─────┘│
        └─┬─┘  └─────┘      │
          │                 │
          │10  ┌─────┐ 10   │
          └───►│A_B_C├──────┘
               └─────┘
**/
pub fn dinic_test_basic(){
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

#[test]
/**
          ┌─┐ 10    ┌─┐ 10     ┌─┐  5
  ┌──────►│A├──────►│B├───────►│C├───────┐
  │       └▲┘       └┬┘        └─┘       │
5 │        │      25 │    15             │
  │      15│         ├──────────┐        │
 ┌┼┐ 10   ┌┼┐  20   ┌▼┐ 30     ┌▼┐ 15   ┌▼┐
 │S├─────►│D├──────►│E├───────►│F├─────►│T│
 └┬┘      └─┘       └┬┘    ┌──►└┬┘      └▲┘
  │             5    │  20 │    │ 15     │
  │ 15    ┌──────────┘     │    │        │
  │      ┌▼┐         ┌─┐───┘   ┌▼┐       │
  └─────►│G├────────►│H│──────►│I├───────┘
         └─┘  25     └─┘   10  └─┘    10
**/

fn dinic_test_advanced(){
    let mut weighted_graph = WeightedGraph::new();
    let s = weighted_graph.graph.create("s", 3);
    let a = weighted_graph.create_and_connect_weighted(s, "a", 5, 1);
    let b = weighted_graph.create_and_connect_weighted(a, "b", 10, 3);
    let c = weighted_graph.create_and_connect_weighted(b, "c", 10, 1);
    let t = weighted_graph.create_and_connect_weighted(c, "t", 5, 0);
    let d = weighted_graph.create_and_connect_weighted(s, "d", 10, 2);
    let e = weighted_graph.create_and_connect_weighted(d, "e", 20, 2);
    let f = weighted_graph.create_and_connect_weighted(e, "f", 30, 2);

    let g = weighted_graph.create_and_connect_weighted(s, "g", 15, 1);
    let h = weighted_graph.create_and_connect_weighted(g, "h", 25, 2);
    let i = weighted_graph.create_and_connect_weighted(h, "i", 10, 1);


    weighted_graph.graph.edge_storage.connect_weighted(b, e, 25);
    weighted_graph.graph.edge_storage.connect_weighted(b, f, 15);

    weighted_graph.graph.edge_storage.connect_weighted(d, a, 15);
    weighted_graph.graph.edge_storage.connect_weighted(e, g, 5);
    weighted_graph.graph.edge_storage.connect_weighted(f, t, 15);
    weighted_graph.graph.edge_storage.connect_weighted(f, i, 15);


    weighted_graph.graph.edge_storage.connect_weighted(h, f, 20);
    weighted_graph.graph.edge_storage.connect_weighted(i, t, 10);

    let dinic_graph = DinicGraph::from(&weighted_graph.graph.vertices, &weighted_graph.graph.edge_storage, s, t);

    let mut snap = HashMap::new();
    snap.insert("sa".to_string(), 5);
    snap.insert("sd".to_string(), 10);
    snap.insert("sg".to_string(), 15);
    snap.insert("ab".to_string(), 5);
    snap.insert("bc".to_string(), 5);
    snap.insert("be".to_string(), 0);
    snap.insert("bf".to_string(), 0);
    snap.insert("ct".to_string(), 5);
    snap.insert("de".to_string(), 10);
    snap.insert("da".to_string(), 0);
    snap.insert("ef".to_string(), 10);
    snap.insert("eg".to_string(), 0);
    snap.insert("ft".to_string(), 15);
    snap.insert("fi".to_string(), 0);
    snap.insert("gh".to_string(), 15);
    snap.insert("hi".to_string(), 10);
    snap.insert("hf".to_string(), 5);
    snap.insert("it".to_string(), 10);

    for (vertex, val) in dinic_graph.vertices.iter().enumerate(){
        for edge in dinic_graph.edge_storage.edges_iter(vertex as VHandle){
            let key = format!("{}{}", *val, weighted_graph.graph.vertices[vh(*edge)]);
            let weight = wgt(*edge);
            assert_eq!(snap.remove(&key), Some(weight));
        }
    }
    assert_eq!(snap.len(), 0);
}