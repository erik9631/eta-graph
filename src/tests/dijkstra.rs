use crate::algorithms::path_finding::{dijkstra};
use crate::traits::{WeightedEdgeConnect};
use crate::weighted_graph::WeightedGraph;


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