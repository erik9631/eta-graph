use crate::graph::{EdgeData, MSize, Vertices};

pub struct TreeHeader{
    pub root: MSize,
    pub parent: MSize,
}
impl TreeHeader{
    ///Number of elements the header takes up in the edge data
    const ELEMENT_COUNT: usize = 2;
}

pub struct Node{
    pub header: TreeHeader,
    pub node: MSize,
}

pub struct NodeData<'a>{
    pub edges: &'a mut EdgeData,
}

pub struct TreeView<'a, T> {
    pub nodes: NodeData<'a>,
    pub values: &'a mut Vertices<T>,
}



impl<'a> NodeData<'a>{
    pub fn new(nodes: &'a mut EdgeData) -> Self {
        return NodeData{
            edges: nodes,
        }
    }
    pub fn get_children(&self, node: &Node) -> &[MSize] {
        match self.edges.edge_data(node.node) {
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


    #[cfg_attr(release, inline(always))]
    pub fn add_child(&mut self, node: &Node, child: Node) -> Node{
        self.edges.connect(node.node, child.node);
        self.edges.set(child.node, node.header.root, 0); // root
        self.edges.set(child.node, node.node, 1); // parent
        return Node::parse(self.edges, child.node);
    }
}

impl Node{
    pub fn parse(edges: &EdgeData, vertex: MSize) -> Node {
        let node_result = edges.edge_data(vertex);
        if node_result.is_err() {
            panic!("Vertex not found!");
        }

        let nodes = node_result.ok().unwrap();
        if nodes.len() == 0 {
            panic!("Not a tree structure! Missing parent or root!");
        }

        return Node{
                header: TreeHeader{
                root: nodes[0],
                parent: nodes[1],
            },
            node: vertex,
        };
    }

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
    pub fn node(&self, vertex: MSize) -> Node {
        return Node::parse(self.nodes.edges, vertex);
    }

    fn create_vertex(&mut self, val: T) -> MSize {
        self.values.push(val);
        self.nodes.edges.create_vertex();
        let vertex = self.values.len() -1;
        return vertex as MSize;
    }

    pub fn create_node(&mut self, val: T) -> Node {
        let vertex = self.create_vertex(val);

        self.nodes.edges.connect(vertex, vertex); // root
        self.nodes.edges.connect(vertex, EdgeData::NONE); // parent

        return Node::parse(self.nodes.edges, vertex);
    }

    pub fn create_child(&mut self, node: &Node, val: T) -> Node {
        let vertex = self.create_vertex(val);

        self.nodes.edges.connect(vertex, node.header.root); // root
        self.nodes.edges.connect(vertex, node.node); // parent
        self.nodes.edges.connect(node.node, vertex); // child

        return Node::parse(self.nodes.edges, vertex);
    }
}