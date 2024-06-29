use crate::graph;
use crate::views::tree::TreeView;

#[test]
pub fn tree_view_test(){
    let mut graph = graph::Graph::new();
    let mut tree_view = graph.tree_view();

    let root = tree_view.create_node("root");
    let child1 = tree_view.create_child(&root, "child1");
    let child2 = tree_view.create_child(&root, "child2");
    let child3 = tree_view.create_child(&root, "child3");
    let child1_1 = tree_view.create_child(&child1, "child1_1");
    let child1_2 = tree_view.create_child(&child1, "child1_2");

    let child1_2_1 = tree_view.create_child(&child1_2, "child1_2_1");

    assert_eq!(tree_view.values[root.node], "root");

    assert_eq!(tree_view.values[child1.node], "child1");
    assert_eq!(tree_view.values[child1.parent], "root");
    assert_eq!(tree_view.values[child1.root], "root");

    assert_eq!(tree_view.values[child2.node], "child2");
    assert_eq!(tree_view.values[child2.parent], "root");
    assert_eq!(tree_view.values[child2.root], "root");

    assert_eq!(tree_view.values[child3.node], "child3");
    assert_eq!(tree_view.values[child3.parent], "root");
    assert_eq!(tree_view.values[child3.root], "root");

    assert_eq!(tree_view.values[child1_1.node], "child1_1");
    assert_eq!(tree_view.values[child1_1.parent], "child1");
    assert_eq!(tree_view.values[child1_1.root], "root");

    assert_eq!(tree_view.values[child1_2.node], "child1_2");
    assert_eq!(tree_view.values[child1_2.parent], "child1");
    assert_eq!(tree_view.values[child1_2.root], "root");

    assert_eq!(tree_view.values[child1_2_1.node], "child1_2_1");
    assert_eq!(tree_view.values[child1_2_1.parent], "child1_2");
    assert_eq!(tree_view.values[child1_2_1.root], "root");

}