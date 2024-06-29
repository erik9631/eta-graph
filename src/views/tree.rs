use std::mem::transmute;
use crate::graph::{EdgeData, Graph, Vertices};

pub struct Node{
    pub root: usize,
    pub parent: usize,
    pub node: usize,
}

impl Node{
    pub fn parse(edges: &EdgeData, vertex: usize) -> Option<Node> {
        let node_result = edges.edges(vertex);
        if node_result.is_err() {
            panic!("Vertex not found!");
        }

        let nodes = node_result.ok().unwrap();
        if nodes.len() == 0 {
            return None;
        }

        return Some(Node{
            root: nodes[0],
            parent: nodes[1],
            node: vertex,
        });
    }

}

pub struct TreeView<'a, T> {
    pub nodes: &'a mut EdgeData,
    pub values: &'a mut Vertices<T>,
}

impl <'a, T> TreeView<'a, T> {
    #[cfg_attr(release, inline(always))]
    pub fn new(edges: &'a mut EdgeData, vertices: &'a mut Vertices<T>) -> Self {
        return TreeView{
            nodes: edges,
            values: vertices,
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn node(&self, vertex: usize) -> Option<Node> {
        return Node::parse(self.nodes, vertex);
    }

    pub fn create_node(&mut self, val: T) -> Node {
        self.values.push(val);
        self.nodes.create_vertex();
        let vertex = self.values.len() -1;

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, EdgeData::NONE); // parent

        return Node::parse(self.nodes, vertex).unwrap();
    }

    pub fn create_child(&mut self, node: &Node, val: T) -> Node {
        self.values.push(val);
        self.nodes.create_vertex();
        let vertex = self.values.len() -1;

        self.nodes.connect(vertex, node.root); // root
        self.nodes.connect(vertex, node.node); // parent
        return Node::parse(self.nodes, vertex).unwrap();
    }

    #[cfg_attr(release, inline(always))]
    pub fn add_child(&mut self, node: &Node, child: usize) {
        self.nodes.connect(node.node, child);
        self.nodes.connect(child, node.root); // root
        self.nodes.connect(child, node.node); // parent
    }

    // pub fn get_children(&self, node: &Node) -> &[usize] {
    //
    // }
}