
use crate::handles::{NONE, Slot, vh, vh_pack};
use crate::handles::types::{PackedEdge, VHandle};
use crate::traits::{Operate, Store, StoreVertex, Visit};
use crate::vertex_storage::VertexStorage;

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
    EdgeStorageType: Store + Operate + Visit,
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(edges: &'a mut EdgeStorageType, vertices: &'a mut VertexStorageType) -> Self {
        return TreeView{
            nodes: edges,
            values: vertices,

        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_children(&self, parent: VHandle) -> &[PackedEdge] {
        return self.nodes.edges_offset(parent, TREE_HEADER_ELEMENTS);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn add_child(&mut self, parent: VHandle, child: VHandle){
        self.nodes.connect(parent, child);
        self.nodes.set(child, vh_pack(parent), PARENT_OFFSET);
        self.nodes.set(child, vh_pack(self.get_root(parent)), ROOT_OFFSET);
    }

    fn create_vertex(&mut self, val: VertexType) -> VHandle {
        self.values.push(val);
        self.nodes.extend_edge_storage(0);
        let vertex = self.values.len() -1;
        return vertex as VHandle;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_root(&self, vertex: VHandle) -> VHandle {
        return vh(self.nodes.get(vertex, 0));
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_parent(&self, vertex: VHandle) -> VHandle {
        return vh(self.nodes.get(vertex, 1));
    }

    pub fn create_node(&mut self, val: VertexType) -> VHandle {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, NONE); // parent

        return vertex;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_child(&mut self, parent: VHandle, val: VertexType) -> VHandle {
        let child = self.create_node(val);
        self.add_child(parent, child);
        return child;
    }
}