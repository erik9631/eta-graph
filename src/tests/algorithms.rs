use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::Resume;
use crate::algorithms::max_flow::DinicGraph;
use crate::graph::Graph;
use crate::handles::types::{VHandle, Weight};
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
pub fn vertices_to_dinic_test(){
    let mut graph = WeightedGraph::new();
    let data_size: VHandle = 2000;
    for i in 0..data_size {
        graph.graph.create_leaf(i);
    }

    let dinic_graph = DinicGraph::from(&graph);

    assert_eq!(dinic_graph.flow_data.len(), data_size as usize);
    assert_eq!(dinic_graph.weighted_graph.graph.vertices.len(), data_size as usize);
    for zipped_iter in dinic_graph.iter_zip(){
        let (vertex, dinic_vertex) = zipped_iter;
        assert_eq!(dinic_vertex.flow, 0);
        assert_eq!(dinic_vertex.level, 0);
        assert_eq!(dinic_vertex.sub_sum, 0);
    }
}