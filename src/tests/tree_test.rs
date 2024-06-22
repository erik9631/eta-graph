use crate::tree;
use crate::tree::{connect, create, edges_len, edges_capacity, get};

#[test]
pub fn graph_init_test() {
    let mut graph = tree::Graph::<i32>::new();
    let first = create(&mut graph, 23);
    let second = create(&mut graph, 24);
    let third = create(&mut graph, 25);

    assert_eq!(*get(&graph, first), 23);
    assert_eq!(*get(&graph, second), 24);
    assert_eq!(*get(&graph, third), 25);
    assert_eq!(edges_capacity(&graph), 33);

    assert_eq!(edges_len(&graph, first), 0);
    assert_eq!(edges_len(&graph, second), 0);
    assert_eq!(edges_len(&graph, third), 0);
}
#[test]
pub fn graph_relationship_test(){
    let mut graph = tree::Graph::<i32>::new();
    let first = create(&mut graph, 23);
    let second = create(&mut graph, 24);
    let third = create(&mut graph, 25);

    connect(&mut graph, first, second);
    connect(&mut graph, second, third);

    assert_eq!(edges_len(&graph, first), 1);
    assert_eq!(edges_len(&graph, second), 1);
    assert_eq!(edges_len(&graph, third), 0);
}