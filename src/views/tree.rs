use crate::graph::{EdgeData, Header, MSize, Vertices};

#[repr(C)]
impl TreeHeader{
    ///Number of elements the header takes up in the edge data
    const ELEMENT_COUNT: usize = 2;
}

pub struct TreeView<'a, T> {
    pub nodes: &'a mut EdgeData,
    pub values: &'a mut Vertices<T>,
}



impl<'a> NodeData<'a>{
    pub fn new(nodes: &'a mut EdgeData) -> Self {
        return NodeData{
            edges: nodes,
        }
    }
    pub fn get_children(&self, node: &Node) -> &[MSize] {
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


    #[cfg_attr(release, inline(always))]
    pub fn add_child(&mut self, node: &Node, child: Node) -> Node{
        self.edges.connect(node.node, child.node);
        self.edges.set(child.node, node.header.root, 0); // root
        self.edges.set(child.node, node.node, 1); // parent
        return Node::parse(self.edges, child.node);
    }
}

impl Node{
    // TODO Reiterate error handling
    pub fn parse(edges: &EdgeData, vertex: MSize) -> Node {
        let node_result = edges.edges(vertex);
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
        self.nodes.create_vertex(0);
        let vertex = self.values.len() -1;
        return vertex as MSize;
    }

    pub fn get_root()

    pub fn create_node(&mut self, val: T) -> MSize {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, EdgeData::NONE); // parent

        return vertex;
    }

    pub fn create_child(&mut self, node: MSize, val: T) -> Node {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, node.header.root); // root
        self.nodes.connect(vertex, node.node); // parent
        self.nodes.connect(node.node, vertex); // child

        return Node::parse(self.nodes.edges, vertex);
    }
}