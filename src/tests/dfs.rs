use crate::algorithms::dfs_bfs::ControlFlow::{End, Resume};
use crate::algorithms::dfs_bfs::dfs;
use crate::graph::Graph;
use crate::handles::{vh, vh_pack};
use crate::traits::StoreVertex;

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

    dfs(&mut graph.edge_storage, vh_pack(root), graph.vertices.len(), |handle|{
        assert_eq!(graph.vertices[vh(*handle)], snap.pop().unwrap());
        Resume
    }, |handle|{
        assert_eq!(graph.vertices[vh(*handle)], snap2.pop().unwrap());
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

    dfs(&mut graph.edge_storage, vh_pack(root), graph.vertices.len(), |handle|{
        if snap.is_empty() {
            return End;
        }
        assert_eq!(graph.vertices[vh(*handle)], snap.pop().unwrap());
        Resume
    }, |handle|{
        assert_eq!(graph.vertices[vh(*handle)], snap2.pop().unwrap());
    });

    assert_eq!(snap.len(), 0);
    assert_eq!(snap2.len(), 0);

}
