use crate::graph;



#[test]
pub fn tree_view_create_child_test(){
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    let child1 = tree_view.create_child(root, "child1");
    let child2 = tree_view.create_child(root, "child2");
    let child3 = tree_view.create_child(root, "child3");
    let child1_1 = tree_view.create_child(child1, "child1_1");
    let child1_2 = tree_view.create_child(child1, "child1_2");

    let child1_2_1 = tree_view.create_child(child1_2, "child1_2_1");

    assert_eq!(tree_view.values[root], "root");

    assert_eq!(tree_view.values[child1], "child1");
    assert_eq!(tree_view.values[tree_view.get_parent(child1)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child1)], "root");

    assert_eq!(tree_view.values[child2], "child2");
    assert_eq!(tree_view.values[child2.header.parent], "root");
    assert_eq!(tree_view.values[child2.header.root], "root");

    assert_eq!(tree_view.values[child3.node], "child3");
    assert_eq!(tree_view.values[child3.header.parent], "root");
    assert_eq!(tree_view.values[child3.header.root], "root");

    assert_eq!(tree_view.values[child1_1.node], "child1_1");
    assert_eq!(tree_view.values[child1_1.header.parent], "child1");
    assert_eq!(tree_view.values[child1_1.header.root], "root");

    assert_eq!(tree_view.values[child1_2.node], "child1_2");
    assert_eq!(tree_view.values[child1_2.header.parent], "child1");
    assert_eq!(tree_view.values[child1_2.header.root], "root");

    assert_eq!(tree_view.values[child1_2_1.node], "child1_2_1");
    assert_eq!(tree_view.values[child1_2_1.header.parent], "child1_2");
    assert_eq!(tree_view.values[child1_2_1.header.root], "root");
}


#[test]
pub fn node_test(){
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    tree_view.create_child(&root, "child1");
    tree_view.create_child(&root, "child2");
    let child3 = tree_view.create_child(&root, "child3");
    let children = tree_view.nodes.get_children(&root);
    assert_eq!(children.len(), 3);

    let child3_parsed = tree_view.node(child3.node);
    assert_eq!(child3_parsed.header.root, root.node);
    assert_eq!(child3_parsed.header.parent, root.node);
    assert_eq!(child3_parsed.node, child3.node);
}
#[should_panic(expected = "Vertex not found!")]
#[test]
pub fn node_test_fail(){
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    tree_view.create_child(&root, "child1");
    tree_view.create_child(&root, "child2");
    tree_view.create_child(&root, "child3");
    let children = tree_view.nodes.get_children(&root);
    assert_eq!(children.len(), 3);

    tree_view.node(100); // Should panic
}

#[test]
pub fn tree_view_add_child_test() {
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    let child1 = tree_view.create_node("child1");
    let child2 = tree_view.create_node( "child2");
    let child3 = tree_view.create_node("child3");

    let child1_1 = tree_view.create_node("child1_1");
    let child1_2 = tree_view.create_node("child1_2");

    let child1 = tree_view.nodes.add_child(&root, child1);
    let child2 = tree_view.nodes.add_child(&root, child2);
    let child3 = tree_view.nodes.add_child(&root, child3);

    let children = tree_view.nodes.get_children(&root);
    assert_eq!(children.len(), 3);

    let child1_1 = tree_view.nodes.add_child(&child1, child1_1);
    let child1_2= tree_view.nodes.add_child(&child1, child1_2);

    let children = tree_view.nodes.get_children(&child1);
    assert_eq!(children.len(), 2);

    assert_eq!(tree_view.values[root.node], "root");

    assert_eq!(tree_view.values[child1.node], "child1");
    assert_eq!(tree_view.values[child1.header.parent], "root");
    assert_eq!(tree_view.values[child1.header.root], "root");

    assert_eq!(tree_view.values[child2.node], "child2");
    assert_eq!(tree_view.values[child2.header.parent], "root");
    assert_eq!(tree_view.values[child2.header.root], "root");

    assert_eq!(tree_view.values[child3.node], "child3");
    assert_eq!(tree_view.values[child3.header.parent], "root");
    assert_eq!(tree_view.values[child3.header.root], "root");

    assert_eq!(tree_view.values[child1_1.node], "child1_1");
    assert_eq!(tree_view.values[child1_1.header.parent], "child1");
    assert_eq!(tree_view.values[child1_1.header.root], "root");

    assert_eq!(tree_view.values[child1_2.node], "child1_2");
    assert_eq!(tree_view.values[child1_2.header.parent], "child1");
    assert_eq!(tree_view.values[child1_2.header.root], "root");
}
#[test]
pub fn get_children_test(){
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    tree_view.create_child(&root, "child1");
    tree_view.create_child(&root, "child2");
    tree_view.create_child(&root, "child3");

    let children = tree_view.nodes.get_children(&root);
    assert_eq!(children.len(), 3);
    for (i, child) in children.iter().enumerate() {
        match i {
            0 => assert_eq!(tree_view.values[*child], "child1"),
            1 => assert_eq!(tree_view.values[*child], "child2"),
            2 => assert_eq!(tree_view.values[*child], "child3"),
            _ => continue,
        }
    }
}
