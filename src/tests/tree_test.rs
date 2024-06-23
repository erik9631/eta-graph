use crate::tree;
use crate::tree::{connect, create, edges_len, edges_capacity, value, create_and_connect, edges, edges_mut, value_mut, get_from_vertices_mut, edges, Graph};

#[test]
pub fn graph_init_test() {
    let mut graph = tree::Graph::<i32>::new();
    let first = create(&mut graph, 23);
    let second = create(&mut graph, 24);
    let third = create(&mut graph, 25);

    assert_eq!(*value(&graph.vertices, first), 23);
    assert_eq!(*value(&graph.vertices, second), 24);
    assert_eq!(*value(&graph.vertices, third), 25);
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
    create_and_connect(&mut graph, e1, "1.1");
    create_and_connect(&mut graph, e1, "1.2");
    create_and_connect(&mut graph, e1, "1.3");

    let e2 = create_and_connect(&mut graph, entry, "2");
    let e21= create_and_connect(&mut graph, e2, "2.1");


    assert_eq!(*value(&graph.vertices, entry), "first");
    assert_eq!(edges_len(&graph, entry), 2);

    assert_eq!(*value(&graph.vertices, e1), "1");
    assert_eq!(edges_len(&graph, e1), 3);

    assert_eq!(*value(&graph.vertices, e2), "2");
    assert_eq!(edges_len(&graph, e2), 1);

    assert_eq!(*value(&graph.vertices, e21), "2.1");
    assert_eq!(edges_len(&graph, e21), 0);


    let entry_edges = edges(&graph, entry, 0);
    assert_eq!(entry_edges.is_err(), false);

    let entry_edges = entry_edges.ok().unwrap();
    assert_eq!(entry_edges.len(), 2);

    for edge in entry_edges {
        match *edge {
            1 => assert_eq!(*value(&graph.vertices, *edge), "1"),
            2 => assert_eq!(*value(&graph.vertices, *edge), "2"),
            _ => continue,
        }
    }

    let e1_result = edges(&graph, e1, 0);
    assert_eq!(e1_result.is_err(), false);

    let e1_edges = e1_result.ok().unwrap();
    assert_eq!(e1_edges.len(), 3);

    for (idx, edge) in e1_edges.iter().enumerate() {
        match idx {
            0 => assert_eq!(*value(&graph.vertices, *edge), "1.1"),
            1 => assert_eq!(*value(&graph.vertices, *edge), "1.2"),
            2 => assert_eq!(*value(&graph.vertices, *edge), "1.3"),
            _ => continue,
        }
    }

    //Mutability check
    let e2_edges = edges(&graph.indices, &graph.edges, e2, 0);
    //let e2_edges = graph.edges(e2, 0);
    assert_eq!(e2_edges.is_err(), false);

    let e2_edges = e2_edges.ok().unwrap();
    assert_eq!(e2_edges.len(), 1);

    for edge in e2_edges {
        match *edge {
            0 =>{
                //*graph.get_vertex_mut(*edge) = "2.2.edited";
                *get_from_vertices_mut(&mut graph.vertices, *edge) = "2.2.edited";
                assert_eq!(*value(&graph.vertices, *edge), "2.2.edited");
            }
            _ => continue,
        }
    }

}
