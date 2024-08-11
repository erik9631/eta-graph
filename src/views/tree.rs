
use crate::handles::{NONE, Slot, vh, vh_pack};
use crate::handles::types::{Edge, VHandle};
use crate::traits::{GraphOperate, EdgeStore, StoreVertex};

pub struct TreeView<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,{
    pub nodes: &'a mut EdgeStorageType,
    pub values: &'a mut VertexStorageType,
}

const TREE_HEADER_ELEMENTS: Slot = 2;
const ROOT_OFFSET: Slot = 0;
const PARENT_OFFSET: Slot = 1;



impl <'a, VertexType, VertexStorageType, EdgeStorageType> TreeView<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: EdgeStore + GraphOperate,
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(edges: &'a mut EdgeStorageType, vertices: &'a mut VertexStorageType) -> Self {
        TreeView{
            nodes: edges,
            values: vertices,

        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_children(&self, parent: VHandle) -> &[Edge] {
        return self.nodes.edges_offset(parent, TREE_HEADER_ELEMENTS);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn add_child(&mut self, parent: VHandle, child: VHandle){
        self.nodes.connect(parent, child);
        let child_edge = self.nodes.get_edges_index(child);
        self.nodes[child_edge + PARENT_OFFSET] = vh_pack(parent);
        self.nodes[child_edge + ROOT_OFFSET] = vh_pack(self.get_root(parent));
    }

    fn create_vertex(&mut self, val: VertexType) -> VHandle {
        self.values.push(val);
        self.nodes.create_edges_entry(0);
        let vertex = self.values.len() -1;
        vertex as VHandle
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_root(&self, vertex: VHandle) -> VHandle {
        vh(self.nodes[self.nodes.get_edges_index(vertex) + ROOT_OFFSET])
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_parent(&self, vertex: VHandle) -> VHandle {
        vh(self.nodes[self.nodes.get_edges_index(vertex) + PARENT_OFFSET])
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