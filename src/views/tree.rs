use crate::graph;
use crate::graph::{EdgeData, Vertices};
struct TreeHeader {
    pub parent: usize,
    pub children: Vec<usize>,
}

struct TreeView<'a, T> {
    pub edges: &'a mut EdgeData,
}

impl <'a, T> TreeView<'a, T> {
    pub fn new(edges: &'a mut EdgeData) -> Self {
        return TreeView{
            edges,
        }
    }
    #[cfg_attr(release, inline(always))]
    fn get_tree_header(&self, vertex: usize) -> &mut TreeHeader {
        let parent = self.get_parent(vertex);
        let children = self.get_children(vertex);
        return TreeHeader{
            parent: parent.unwrap_or(EdgeData::NONE),
            children,
        }
    }

    pub fn get_parent(&self, vertex: usize) -> Option<usize> {
        let node_result = self.edges.edges(vertex);
        if node_result.is_err() {
            panic!("Vertex not found!");
        }

        let nodes = node_result.ok().unwrap();
        if nodes.len() == 0 {
            return None;
        }

        if nodes[0] == EdgeData::NONE {
            return None;
        }

        return Some(nodes[0]);
    }
}