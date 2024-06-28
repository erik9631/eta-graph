use std::mem::transmute;
use crate::graph::{EdgeData};
struct TreeHeader {
    pub parent: usize,
    pub root: usize,
}

impl TreeHeader {
    #[inline]
    pub fn parse(data: &[usize]) -> &TreeHeader{
        unsafe {
            let header = data.as_ptr();
            let header_struct = transmute(header);
            return header_struct;
        }
    }
    #[inline]
    pub fn parse_mut(data: &mut [usize]) -> &mut TreeHeader{
        unsafe {
            let header = &*(data.as_ptr() as *mut TreeHeader);
            let header_struct = transmute(header);
            return header_struct;
        }

    }
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

    pub fn header(&self, vertex: usize) -> Option<&TreeHeader> {
        let node_result = self.edges.edges(vertex);
        if node_result.is_err() {
            panic!("Vertex not found!");
        }

        let nodes = node_result.ok().unwrap();
        if nodes.len() == 0 {
            return None;
        }

        let header = TreeHeader::parse(nodes);

        return Some(header);
    }
}