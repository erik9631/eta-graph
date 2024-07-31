use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::Resume;
use crate::algorithms::max_flow::DinicGraphView;
use crate::graph::Graph;
use crate::handles::types::{VHandle, Weight};
use crate::traits::GraphOperate;
use crate::weighted_graph::WeightedGraph;

#[test]
pub fn graph_bfs_test(){
    let mut graph = Graph::new();
    let root = graph.create("root", 3);
    let a = graph.create_and_connect(root, "a", 3);
    let b = graph.create_and_connect(root, "b", 2);
    graph.create_and_connect_leaf(root, "c");

    graph.create_and_connect_leaf(a, "a_a");
    graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a", 1);
    graph.create_and_connect_leaf(b, "b_b");

    graph.create_and_connect_leaf(b_a, "b_a_a");
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

    bfs(&mut graph.edges, root, graph.vertices.len(), |_edges, handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[handle], val.0);
        assert_eq!(layer, val.1);
        Resume
    });
}

#[test]
pub fn graph_bfs_test_cyclic(){
    let mut graph = Graph::with_reserve(2);
    let root = graph.create("root", 3);
    let a = graph.create_and_connect(root, "a", 3);
    let b = graph.create_and_connect(root, "b", 2);
    let c = graph.create_and_connect_leaf(root, "c");
    graph.edges.connect(c, root);

    graph.create_and_connect_leaf(a, "a_a");
    let a_b = graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a", 1);
    graph.create_and_connect_leaf(b, "b_b");

    let b_a_a = graph.create_and_connect_leaf(b_a, "b_a_a");
    graph.edges.connect(a_b, b_a_a);
    graph.edges.connect(b_a_a, b_a);

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

    bfs(&mut graph.edges, root, graph.vertices.len(), |_edges, handle, layer|{
        let val = snap.pop().unwrap();
        assert_eq!(graph.vertices[handle], val.0);
        assert_eq!(layer, val.1);
        Resume
    });
}

#[test]
pub fn graph_dfs_test(){
    let mut graph = Graph::new();
    let root = graph.create("root", 3);
    let a = graph.create_and_connect(root, "a", 3);
    let b = graph.create_and_connect(root, "b", 2);
    graph.create_and_connect_leaf(root, "c");

    graph.create_and_connect_leaf(a, "a_a");
    graph.create_and_connect_leaf(a, "a_b");
    graph.create_and_connect_leaf(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a", 1);
    graph.create_and_connect_leaf(b, "b_b");

    graph.create_and_connect_leaf(b_a, "b_a_a");

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

    dfs(&mut graph.edges, root, graph.vertices.len(), |_edges, handle|{
        assert_eq!(graph.vertices[handle], snap.pop().unwrap());
        Resume
    }, |_edges, handle|{
        assert_eq!(graph.vertices[handle], snap2.pop().unwrap());
    });

}


#[test]
pub fn graph_to_dinic_test(){
    let mut graph = WeightedGraph::new();
    let data_size: VHandle = 2000;
    for i in 0..data_size {
        graph.graph.create_leaf(i);
    }

    let dinic_graph = DinicGraphView::from(&mut graph);

    assert_eq!(dinic_graph.flow_data.len(), data_size as usize);
    assert_eq!(dinic_graph.weighted_graph.graph.vertices.len(), data_size as usize);
    for zipped_iter in dinic_graph.iter_zip(){
        let (vertex, dinic_vertex) = zipped_iter;
        assert_eq!(dinic_vertex.flow, 0);
        assert_eq!(dinic_vertex.level, 0);
        assert_eq!(dinic_vertex.sub_sum, 0);
    }
}

#[test]
pub fn dinic_level_test(){
    let mut graph = WeightedGraph::new();
    // Write test for layering
}