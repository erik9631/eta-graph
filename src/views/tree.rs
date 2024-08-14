
use crate::handles::{NONE, eh_pack, eh};
use crate::handles::types::{Edge, VHandle};
use crate::traits::{EdgeConnect, EdgeStore, StoreVertex};

pub struct Tree<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,{
    pub nodes: &'a mut EdgeStorageType,
    pub values: &'a mut VertexStorageType,
}
const ROOT_OFFSET: usize = 0;
const PARENT_OFFSET: usize = 1;
const TREE_HEADER_ELEMENTS: usize = 2;



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
    pub fn get_children(&self, parent: VHandle) -> &[Edge] {
        &self.nodes.vertex_as_slice(parent)[TREE_HEADER_ELEMENTS as usize..]
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn add_child(&mut self, parent: VHandle, child: VHandle){
        self.nodes.connect(parent, child);
        let child_edge = self.nodes.vertex_index(child);
        self.nodes[child_edge + PARENT_OFFSET] = eh_pack(parent);
        self.nodes[child_edge + ROOT_OFFSET] = eh_pack(self.get_root(parent));
    }

    fn create_vertex(&mut self, val: VertexType) -> VHandle {
        self.values.push(val);
        self.nodes.create_vertex(0);
        let vertex = self.values.len() -1;
        vertex as VHandle
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_root(&self, vertex: VHandle) -> VHandle {
        eh(self.nodes[self.nodes.vertex_index(vertex) + ROOT_OFFSET])
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_parent(&self, vertex: VHandle) -> VHandle {
        eh(self.nodes[self.nodes.vertex_index(vertex) + PARENT_OFFSET])
    }

    pub fn create_node(&mut self, val: VertexType) -> VHandle {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, NONE); // parent

        vertex
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_child(&mut self, parent: VHandle, val: VertexType) -> VHandle {
        let child = self.create_node(val);
        self.add_child(parent, child);
        child
    }
}