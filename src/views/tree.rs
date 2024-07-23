use crate::edge_storage::{EdgeStorage};
use crate::graph::{Vertices};
use crate::size::VHandle;
use crate::traits::{EdgeOperator, EdgeStore, EdgeStoreMut};

pub struct TreeView<'a, T> {
    pub nodes: &'a mut EdgeStorage,
    pub values: &'a mut Vertices<T>,
}

const TREE_HEADER_ELEMENTS: VHandle = 2;
const ROOT_OFFSET: usize = 0;
const PARENT_OFFSET: usize = 1;



impl <'a, T> TreeView<'a, T> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(edges: &'a mut EdgeStorage, vertices: &'a mut Vertices<T>) -> Self {
        return TreeView{
            nodes: edges,
            values: vertices,
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_children(&self, parent: VHandle) -> &[VHandle] {
        return self.nodes.edges_offset(parent, TREE_HEADER_ELEMENTS as usize);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn add_child(&mut self, parent: VHandle, child: VHandle){
        self.nodes.connect(parent, child);
        self.nodes.set(child, parent, PARENT_OFFSET);
        self.nodes.set(child, self.get_root(parent), ROOT_OFFSET);
    }

    fn create_vertex(&mut self, val: T) -> VHandle {
        self.values.push(val);
        self.nodes.extend_edge_storage(0);
        let vertex = self.values.len() -1;
        return vertex as VHandle;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_root(&self, vertex: VHandle) -> VHandle {
        return self.nodes.get(vertex, 0);
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_parent(&self, vertex: VHandle) -> VHandle {
        return self.nodes.get(vertex, 1);
    }

    pub fn create_node(&mut self, val: T) -> VHandle {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, EdgeStorage::NONE); // parent

        return vertex;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_child(&mut self, parent: VHandle, val: T) -> VHandle {
        let child = self.create_node(val);
        self.add_child(parent, child);
        return child;
    }
}