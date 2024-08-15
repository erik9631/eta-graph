use crate::graph;
use crate::handles::vh;


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
    assert_eq!(tree_view.values[tree_view.get_parent(child2)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child2)], "root");

    assert_eq!(tree_view.values[child3], "child3");
    assert_eq!(tree_view.values[tree_view.get_parent(child3)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child3)], "root");

    assert_eq!(tree_view.values[child1_1], "child1_1");
    assert_eq!(tree_view.values[tree_view.get_parent(child1_1)], "child1");
    assert_eq!(tree_view.values[tree_view.get_root(child1_1)], "root");

    assert_eq!(tree_view.values[child1_2], "child1_2");
    assert_eq!(tree_view.values[tree_view.get_parent(child1_2)], "child1");
    assert_eq!(tree_view.values[tree_view.get_root(child1_2)], "root");

    assert_eq!(tree_view.values[child1_2_1], "child1_2_1");
    assert_eq!(tree_view.values[tree_view.get_parent(child1_2_1)], "child1_2");
    assert_eq!(tree_view.values[tree_view.get_root(child1_2_1)], "root");
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

    tree_view.add_child(root, child1);
    tree_view.add_child(root, child2);
    tree_view.add_child(root, child3);

    let children = tree_view.get_children(root);
    assert_eq!(children.len(), 3);

    tree_view.add_child(child1, child1_1);
    tree_view.add_child(child1, child1_2);

    let children = tree_view.get_children(child1);
    assert_eq!(children.len(), 2);

    assert_eq!(tree_view.values[root], "root");

    assert_eq!(tree_view.values[child1], "child1");
    assert_eq!(tree_view.values[tree_view.get_parent(child1)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child1)], "root");

    assert_eq!(tree_view.values[child2], "child2");
    assert_eq!(tree_view.values[tree_view.get_parent(child2)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child2)], "root");

    assert_eq!(tree_view.values[child3], "child3");
    assert_eq!(tree_view.values[tree_view.get_parent(child3)], "root");
    assert_eq!(tree_view.values[tree_view.get_root(child3)], "root");

    assert_eq!(tree_view.values[child1_1], "child1_1");
    assert_eq!(tree_view.values[tree_view.get_parent(child1_1)], "child1");
    assert_eq!(tree_view.values[tree_view.get_root(child1_1)], "root");

    assert_eq!(tree_view.values[child1_2], "child1_2");
    assert_eq!(tree_view.values[tree_view.get_parent(child1_2)], "child1");
    assert_eq!(tree_view.values[tree_view.get_root(child1_2)], "root");
}
#[test]
pub fn get_children_test(){
    let mut graph = graph::Graph::new_large();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    tree_view.create_child(root, "child1");
    tree_view.create_child(root, "child2");
    tree_view.create_child(root, "child3");

    let children = tree_view.get_children(root);
    assert_eq!(children.len(), 3);
    for (i, child) in children.iter().enumerate() {
        match i {
            0 => assert_eq!(tree_view.values[vh(*child)], "child1"),
            1 => assert_eq!(tree_view.values[vh(*child)], "child2"),
            2 => assert_eq!(tree_view.values[vh(*child)], "child3"),
            _ => continue,
        }
    }
}
