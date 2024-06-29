use std::mem::{size_of, transmute};
use std::slice::from_raw_parts;
use crate::graph::{EdgeData, Graph, Vertices};

pub struct TreeHeader{
    pub root: usize,
    pub parent: usize,
}
impl TreeHeader{
    ///Number of elements the header takes up in the edge data
    const ELEMENT_COUNT: usize = 2;
}

pub struct Node{
    pub header: TreeHeader,
    pub node: usize,
}

pub struct NodeData<'a>{
    pub edges: &'a mut EdgeData,
}

impl<'a> NodeData{
    pub fn new(nodes: &'a mut EdgeData) -> Self {
        return NodeData{
            edges: nodes,
        }
    }
    pub fn get_children(&self, node: &Node) -> &[usize] {
        match self.edges.edges(node.node) {
            Ok(children_slice) => {
                if children_slice.len() > TreeHeader::ELEMENT_COUNT {
                    return &children_slice[TreeHeader::ELEMENT_COUNT..]
                }
                return &[]
            },
            Err(_) => {
                panic!("Vertex not found!");
            }
        }
    }
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
            header: TreeHeader{
            root: nodes[0],
            parent: nodes[1],
        },
            node: vertex,
        });
    }

}

pub struct TreeView<'a, T> {
    pub nodes: NodeData<'a>,
    pub values: &'a mut Vertices<T>,
}

impl <'a, T> TreeView<'a, T> {
    #[cfg_attr(release, inline(always))]
    pub fn new(edges: &'a mut EdgeData, vertices: &'a mut Vertices<T>) -> Self {
        return TreeView{
            nodes: NodeData::new(edges),
            values: vertices,
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn node(&self, vertex: usize) -> Option<Node> {
        return Node::parse(self.nodes, vertex);
    }

    fn create_vertex(&mut self, val: T) -> usize {
        self.values.push(val);
        self.nodes.create_vertex();
        let vertex = self.values.len() -1;
        return vertex;
    }

    pub fn create_node(&mut self, val: T) -> Node {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, EdgeData::NONE); // parent

        return Node::parse(self.nodes, vertex).unwrap();
    }

    pub fn create_child(&mut self, node: &Node, val: T) -> Node {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, node.header.root); // root
        self.nodes.connect(vertex, node.node); // parent
        return Node::parse(self.nodes, vertex).unwrap();
    }

    #[cfg_attr(release, inline(always))]
    pub fn add_child(&mut self, node: &Node, child: usize) {
        self.nodes.connect(node.node, child);
        self.nodes.connect(child, node.header.root); // root
        self.nodes.connect(child, node.node); // parent
    }
}