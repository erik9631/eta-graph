
use crate::handles::{NONE, Slot, eh_pack, eh};
use crate::handles::types::{Edge, EHandle};
use crate::traits::{EdgeConnect, EdgeStore, StoreVertex};

pub struct Tree<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,{
    pub nodes: &'a mut EdgeStorageType,
    pub values: &'a mut VertexStorageType,
}
const ROOT_OFFSET: Slot = 0;
const PARENT_OFFSET: Slot = 1;
const TREE_HEADER_ELEMENTS: Slot = 2;



impl <'a, VertexType, VertexStorageType, EdgeStorageType> Tree<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: EdgeStore + EdgeConnect,
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(edges: &'a mut EdgeStorageType, vertices: &'a mut VertexStorageType) -> Self {
        Tree {
            nodes: edges,
            values: vertices,

        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_children(&self, parent: EHandle) -> &[Edge] {
        &self.nodes.entry_as_slice(parent)[TREE_HEADER_ELEMENTS as usize..]
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn add_child(&mut self, parent: EHandle, child: EHandle){
        self.nodes.connect(parent, child);
        let child_edge = self.nodes.entry_index(child);
        self.nodes[child_edge + PARENT_OFFSET] = eh_pack(parent);
        self.nodes[child_edge + ROOT_OFFSET] = eh_pack(self.get_root(parent));
    }

    fn create_vertex(&mut self, val: VertexType) -> EHandle {
        self.values.push(val);
        self.nodes.create_entry(0);
        let vertex = self.values.len() -1;
        vertex as EHandle
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_root(&self, vertex: EHandle) -> EHandle {
        eh(self.nodes[self.nodes.entry_index(vertex) + ROOT_OFFSET])
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_parent(&self, vertex: EHandle) -> EHandle {
        eh(self.nodes[self.nodes.entry_index(vertex) + PARENT_OFFSET])
    }

    pub fn create_node(&mut self, val: VertexType) -> EHandle {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, NONE); // parent

        vertex
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_child(&mut self, parent: EHandle, val: VertexType) -> EHandle {
        let child = self.create_node(val);
        self.add_child(parent, child);
        child
    }
}