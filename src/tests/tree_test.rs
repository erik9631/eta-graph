use crate::tree;
use crate::tree::{connect, create, edges_len, edges_capacity, get, create_and_connect, edge};

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
    let mut graph = tree::Graph::<&str>::new();
    let entry = create(&mut graph, "first");
    let e1 = create_and_connect(&mut graph, entry, "1");
    let e2 = create_and_connect(&mut graph, entry, "2");
    create_and_connect(&mut graph, e1, "1.1");
    create_and_connect(&mut graph, e1, "1.2");
    create_and_connect(&mut graph, e1, "1.3");

    assert_eq!(*get(&graph, entry), "first");
    assert_eq!(edges_len(&graph, entry), 2);

    assert_eq!(*get(&graph, e1), "1");
    assert_eq!(edges_len(&graph, e1), 3);

    assert_eq!(*get(&graph, e2), "2");
    assert_eq!(edges_len(&graph, e2), 0);

    // Value check
    for i in 0..edges_len(&graph, entry){
        let edge_index = edge(&graph, entry, i);
        if edge_index.is_none(){
            continue;
        }
        let edge_index = edge_index.unwrap();

        match i {
            0 => assert_eq!(*get(&graph, edge_index), "1"),
            1 => assert_eq!(*get(&graph, edge_index), "2"),
            _ => panic!("Unexpected value")
        }
    }

}
