use crate::graph;

#[test]
pub fn graph_init_test() {
    let mut graph = graph::Graph::new();
    assert_eq!(graph.vertices.len(), 0);
    assert_eq!(graph.edges.len(), 0);

    graph.create(1);
    graph.create(2);
    graph.create(3);

    assert_eq!(graph.vertices.len(), 3);
    assert_eq!(graph.edges.len(), 153);

}

#[test]
pub fn graph_basic_test(){
    let mut graph = graph::Graph::new();
    let a = graph.create("a");
    let b = graph.create("b");
    graph.create("c");

    graph.create_and_connect(a, "a_a");
    graph.create_and_connect(a, "a_b");
    graph.create_and_connect(a, "a_c");

    let b_a = graph.create_and_connect(b, "b_a");
    graph.create_and_connect(b, "b_b");

   graph.create_and_connect(b_a, "b_a_a");

    let a_edges_result = graph.edges.edges(a);
    assert_eq!(a_edges_result.is_err(), false);

    let a_edges = a_edges_result.ok().unwrap();
    assert_eq!(a_edges.len(), 3);

    for edge in a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "a_a"),
            1 => assert_eq!(graph.vertices[*edge], "a_b"),
            2 => assert_eq!(graph.vertices[*edge], "a_c"),
            _ => continue,
        }
    }

    let b_edges_result = graph.edges.edges(b);
    assert_eq!(b_edges_result.is_err(), false);

    let b_edges = b_edges_result.ok().unwrap();
    assert_eq!(b_edges.len(), 2);

    for edge in b_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "b_a"),
            1 => assert_eq!(graph.vertices[*edge], "b_b"),
            _ => continue,
        }
    }

    let b_a_a_edges_result = graph.edges.edges(b_a);
    assert_eq!(b_a_a_edges_result.is_err(), false);

    let b_a_a_edges = b_a_a_edges_result.ok().unwrap();
    assert_eq!(b_a_a_edges.len(), 1);

    for edge in b_a_a_edges {
        match *edge{
            0 => assert_eq!(graph.vertices[*edge], "b_a_a"),
            _ => continue,
        }
    }

}
