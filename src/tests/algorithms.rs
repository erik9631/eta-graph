use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::{End, Resume};
use crate::algorithms::max_flow::{DinicGraph, mark_levels};
use crate::algorithms::path_finding::{dijkstra};
use crate::graph::Graph;
use crate::handles::types::{Edge, Weight};
use crate::handles::{eh, eh_pack, wgt};
use crate::traits::{EdgeStore, EdgeConnect, WeightedEdgeConnect};
use crate::weighted_graph::WeightedGraph;

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

    bfs(&mut graph.edge_storage, eh_pack(root), graph.vertices.len(), |handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[eh(*handle)], val.0);
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

    bfs(&mut graph.edge_storage, eh_pack(root), graph.vertices.len(), |handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[eh(*handle)], val.0);
        assert_eq!(layer, val.1);
        Resume
    });
    assert_eq!(snap.len(), 0);
}

#[test]
pub fn graph_dfs_test(){
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

    let mut snap = vec![
        "c".to_string(),
        "b_b".to_string(),
        "b_a_a".to_string(),
        "b_a".to_string(),
        "b".to_string(),
        "a_c".to_string(),
        "a_b".to_string(),
        "a_a".to_string(),
        "a".to_string(),
        "root".to_string(),
    ];

    let mut snap2 = vec![
        "root".to_string(),
        "c".to_string(),
        "b".to_string(),
        "b_b".to_string(),
        "b_a".to_string(),
        "b_a_a".to_string(),
        "a".to_string(),
        "a_c".to_string(),
        "a_b".to_string(),
        "a_a".to_string(),
    ];

    dfs(&mut graph.edge_storage, eh_pack(root), graph.vertices.len(), |handle|{
        assert_eq!(graph.vertices[eh(*handle)], snap.pop().unwrap());
        Resume
    }, |handle|{
        assert_eq!(graph.vertices[eh(*handle)], snap2.pop().unwrap());
    });

    assert_eq!(snap.len(), 0);
    assert_eq!(snap2.len(), 0);
}

#[test]
pub fn graph_dfs_end_test(){
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

    let mut snap = vec![
        "a_c".to_string(),
        "a_b".to_string(),
        "a_a".to_string(),
        "a".to_string(),
        "root".to_string(),
    ];

    let mut snap2 = vec![
        "root".to_string(),
        "a".to_string(),
        "a_c".to_string(),
        "a_b".to_string(),
        "a_a".to_string(),
    ];

    dfs(&mut graph.edge_storage, eh_pack(root), graph.vertices.len(), |handle|{
        if snap.is_empty() {
            return End;
        }
        assert_eq!(graph.vertices[eh(*handle)], snap.pop().unwrap());
        Resume
    }, |handle|{
        assert_eq!(graph.vertices[eh(*handle)], snap2.pop().unwrap());
    });

    assert_eq!(snap.len(), 0);
    assert_eq!(snap2.len(), 0);

}

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


#[test]
/**
┌────┐         ┌────┐  ┌────┐              ┌────┐
│    │   1     │    │1 │    │ 2            │    │   7
│ A  ├────────►│ C  ├─►│ D  ├──┐           │ F  ├────────┐
└────┘         └────┘  └────┘  ▼           └────┘        ▼
  ▲               ▲          ┌────┐    5     ▲         ┌────┐
  │               │          │    ├──────────┘         │    │
20│               │2         │ E  ├──────────┐         │ T  │
  │               │          └────┘   1      ▼         └────┘
┌─┴──┐          ┌─┴──┐         ▲           ┌────┐        ▲
│    │          │    │         │           │    │        │
│ S  ├─────────►│ B  │ ────────┘           │ G  ├────────┘
└────┘   5      └────┘    4                └────┘
                                                     8
*/
pub fn dijkstra_test_basic() {
    let mut weighted_graph = WeightedGraph::new();
    let s = weighted_graph.graph.create("s", 2);
    let a = weighted_graph.create_and_connect_weighted(s, "a", 20, 1);
    let b = weighted_graph.create_and_connect_weighted(s, "b", 5, 2);
    let c = weighted_graph.create_and_connect_weighted(b, "c", 2, 1);
    weighted_graph.graph.edge_storage.connect_weighted(a, c, 1);
    let d = weighted_graph.create_and_connect_weighted(c, "d", 1, 1);
    let e = weighted_graph.create_and_connect_weighted(d, "e", 2, 2);
    weighted_graph.graph.edge_storage.connect_weighted(b, e, 4);
    let f = weighted_graph.create_and_connect_weighted(e, "f", 5, 1);
    let g = weighted_graph.create_and_connect_weighted(e, "g", 1, 1);
    let t = weighted_graph.create_and_connect_weighted(f, "t", 7, 0);
    weighted_graph.graph.edge_storage.connect_weighted(g,t, 8);

    let mut result = dijkstra(&mut weighted_graph.graph.edge_storage, s, t, weighted_graph.graph.vertices.len());
    if result.is_none(){
        assert!(false, "Path from S to T should exist");
    }
    let mut path = result.unwrap();
    let mut snap = vec![
        "t".to_string(),
        "g".to_string(),
        "e".to_string(),
        "b".to_string(),
        "s".to_string(),
    ];

    while !path.is_empty(){
        let val = snap.pop().unwrap();
        let handle = path.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[handle], val);
    }
}

#[test]
/**
 ┌────┐   1    ┌────┐   1    ┌────┐   3    ┌────┐
 │    ├───────►│    ├───────►│    ├───────►│ G  │
 │ B  │◄───────┤  C │◄───────┤E   │◄───────┤    │
 └┬───┘        └──┬─┘        └┬───┘        └──┬─┘
  │ ▲           ▲ │           │ ▲           ▲ │
  │ │           │ │           │ │           │ │
1 │ │         3 │ │          1│ │           │ │ 1
  │ │           │ │           │ │           │ │
  ▼ │           │ ▼           ▼ │           │ ▼
 ┌──┴─┐        ┌┴───┐        ┌──┴─┐        ┌┴───┐
 │    │◄───────┤    │◄───────┤    │◄───────┤    │
 │ A  ├───────►│ D  ├───────►│ F  ├───────►│ H  │
 └────┘   3    └────┘   3    └────┘   1    └────┘
 */
pub fn dijkstra_test_cyclic(){
    let mut weighted_graph = WeightedGraph::new();
    let a = weighted_graph.graph.create("a", 2);
    let b = weighted_graph.create_and_connect_weighted(a, "b", 1, 2);
    weighted_graph.graph.edge_storage.connect_weighted(b, a, 1);
    let d = weighted_graph.create_and_connect_weighted(a, "d", 3, 3);
    weighted_graph.graph.edge_storage.connect_weighted(d, a, 3);
    let c = weighted_graph.create_and_connect_weighted(b, "c", 1, 3);
    weighted_graph.graph.edge_storage.connect_weighted(c, b, 1);
    let f = weighted_graph.create_and_connect_weighted(d, "f", 3, 3);
    weighted_graph.graph.edge_storage.connect_weighted(f, d, 3);
    let e = weighted_graph.create_and_connect_weighted(c, "e", 1, 3);
    weighted_graph.graph.edge_storage.connect_weighted(e, c, 1);
    let g = weighted_graph.create_and_connect_weighted(e, "g", 3, 2);
    weighted_graph.graph.edge_storage.connect_weighted(g, e, 3);
    let h = weighted_graph.create_and_connect_weighted(f, "h", 1, 2);
    weighted_graph.graph.edge_storage.connect_weighted(h, f, 1);

    weighted_graph.graph.edge_storage.connect_weighted(c, d, 3);
    weighted_graph.graph.edge_storage.connect_weighted(d, c, 3);

    weighted_graph.graph.edge_storage.connect_weighted(e, f, 1);
    weighted_graph.graph.edge_storage.connect_weighted(f, e, 1);

    let mut snap = vec![
        "g".to_string(),
        "e".to_string(),
        "c".to_string(),
        "b".to_string(),
        "a".to_string(),
    ];

    let mut result = dijkstra(&mut weighted_graph.graph.edge_storage, a, g, weighted_graph.graph.vertices.len());
    if result.is_none(){
        assert!(false, "Path from A to G should exist");
    }
    let mut path = result.unwrap();

    while !path.is_empty(){
        let val = snap.pop().unwrap();
        let handle = path.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[handle], val);
    }
}

#[test]
/**
┌────┐   2    ┌────┐   5    ┌────┐
│ A  ├───────►│ B  ├───────►│ C  │
└─┬──┘        └─┬──┘        └─┬──┘
  │             │             │
  │             │             │
 1│            3│            2│
  │             │             │
  │             │             │
  ▼             ▼             ▼
┌────┐   4    ┌────┐   1    ┌────┐
│ D  ├───────►│ E  ├───────►│ F  │
└─┬──┘        └─┬──┘        └────┘
  │             │
  │             │
 6│            2│
  │             │
  │             │
  ▼             ▼
┌────┐   3    ┌────┐
│ G  ├───────►│ H  │
└────┘        └────┘
 */
pub fn dijkstra_test_directed_acyclic() {
    let mut weighted_graph = WeightedGraph::with_reserve(5);
    let a = weighted_graph.graph.create("A", 1);
    let b = weighted_graph.create_and_connect_weighted_0(a, "B", 2);
    let c = weighted_graph.create_and_connect_weighted_0(b, "C", 5);
    let d = weighted_graph.create_and_connect_weighted_0(a, "D", 1);
    let e = weighted_graph.create_and_connect_weighted_0(b, "E", 3);
    let f = weighted_graph.create_and_connect_weighted_0(c, "F", 2);
    let g = weighted_graph.create_and_connect_weighted_0(d, "G", 6);
    let h = weighted_graph.create_and_connect_weighted_0(e, "H", 2);

    weighted_graph.graph.edge_storage.connect_weighted(d, e, 4);
    weighted_graph.graph.edge_storage.connect_weighted(e, f, 1);
    weighted_graph.graph.edge_storage.connect_weighted(g, h, 3);

    // Test path from A to F
    let mut expected_path = vec![
        "F".to_string(),
        "E".to_string(),
        "B".to_string(),
        "A".to_string(),
    ];

    let mut result = dijkstra(&mut weighted_graph.graph.edge_storage, a, f, weighted_graph.graph.vertices.len());
    if result.is_none(){
        assert!(false, "Path from A to F should exist");
    }
    let mut path = result.unwrap();

    while !path.is_empty() {
        let val = expected_path.pop().unwrap();
        let handle = path.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[handle], val);
    }

    // Test path from A to H
    let mut expected_path = vec![
        "H".to_string(),
        "E".to_string(),
        "B".to_string(),
        "A".to_string(),
    ];

    let mut result = dijkstra(&mut weighted_graph.graph.edge_storage, a, h, weighted_graph.graph.vertices.len());
    if result.is_none(){
        assert!(false);
    }
    let mut path = result.unwrap();

    while !path.is_empty(){
        let val = expected_path.pop().unwrap();
        let handle = path.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[handle], val);
    }
}

#[test]
/**
┌────┐   2    ┌────┐   3    ┌────┐
│ A  ├───────►│ B  ├───────►│ C  │
└─┬──┘        └─┬──┘        └────┘
  │             │
 1│            4│
  │             │
  ▼             ▼
┌────┐   2    ┌────┐
│ D  ├───────►│ E  │
└────┘        └────┘

┌────┐
│ F  │  (Disconnected node)
└────┘
 */
pub fn dijkstra_test_with_disconnected_node() {
    let mut weighted_graph = WeightedGraph::with_reserve(5);
    let a = weighted_graph.graph.create("A", 1);
    let b = weighted_graph.create_and_connect_weighted_0(a, "B", 2);
    let c = weighted_graph.create_and_connect_weighted_0(b, "C", 3);
    let d = weighted_graph.create_and_connect_weighted_0(a, "D", 1);
    let e = weighted_graph.create_and_connect_weighted_0(b, "E", 4);
    let f = weighted_graph.graph.create("F", 1);  // Disconnected node

    weighted_graph.graph.edge_storage.connect_weighted(d, e, 2);

    // Test 1: Path from A to C (should exist)
    let mut expected_path = vec!["C".to_string(), "B".to_string(), "A".to_string()];
    let mut result = dijkstra(&mut weighted_graph.graph.edge_storage, a, c, weighted_graph.graph.vertices.len());
    if result.is_none(){
        assert!(false, "Path from A to C should exist");
    }

    let mut path = result.unwrap();
    while !path.is_empty() {
        let val = expected_path.pop().unwrap();
        let handle = path.pop().unwrap();
        assert_eq!(weighted_graph.graph.vertices[handle], val);
    }

    // Test 2: Path from A to F (should not exist)
    let result = dijkstra(&mut weighted_graph.graph.edge_storage, a, f, weighted_graph.graph.vertices.len());
    assert!(result.is_none(), "Path from A to F should not exist");

    // Test 3: Path from F to any other node (should not exist)
    let path = dijkstra(&mut weighted_graph.graph.edge_storage, f, a, weighted_graph.graph.vertices.len());
    assert!(result.is_none(), "Path from F to A should not exist");
}