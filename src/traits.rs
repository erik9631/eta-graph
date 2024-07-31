use std::ops::{Deref, DerefMut, Index, IndexMut};
use crate::handles::Slot;
use crate::handles::types::{PackedEdge, VHandle, Weight};

// TODO add iters for edges

pub trait Transform<T>{
    fn transform(&mut self, transform_fn: fn(&mut [T]));
}

pub trait AsyncTransform<T>: Transform<T>{
    fn async_transform(&mut self, transform_fn: fn(&mut [T]));
}

pub trait StoreVertex: Index<VHandle, Output=Self::VertexType> + IndexMut<VHandle, Output=Self::VertexType>{
    type VertexType;
    fn len(&self) -> usize;
    fn push(&mut self, val: Self::VertexType);
    fn capacity(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::VertexType>;
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VertexType>;
    fn as_slice(&self) -> &[Self::VertexType];
}

pub trait GraphOperate {
    fn add_edges(&mut self, src: VHandle, targets: &[PackedEdge]);
    fn extend_edge_storage(&mut self, size: Slot) -> Slot;
    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle);
    fn connect(&mut self, from: VHandle, to: VHandle);
}

pub trait WeightedGraphOperate {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight);
}

pub trait EdgeVisit {
    fn global_visited_flag(&self) -> Slot;
    fn inc_global_visited_flag(&mut self);
    fn reset_global_visited_flag(&mut self);
    fn visited_flag(&self, vertex: VHandle) -> Slot;
    fn inc_visited_flag(&mut self, vertex: VHandle);
    fn set_visited_flag(&mut self, vertex: VHandle, val: Slot);
}
pub trait EdgeStore {
    fn edges_offset(&self, vertex: VHandle, offset: Slot) -> &[PackedEdge];
    fn edges_ptr_offset(&self, vertex: VHandle, offset: Slot) -> *const PackedEdge;
    fn edges(&self, vertex: VHandle) -> &[PackedEdge];
    fn edges_ptr(&self, vertex: VHandle) -> *const PackedEdge;
    fn len(&self, handle: VHandle) -> Slot;
    fn edge_block_capacity(&self, handle: VHandle) -> Slot;
    fn get(&self, vertex: VHandle, offset: Slot) -> PackedEdge;
    fn edges_mut_offset(&mut self, vertex: VHandle, offset: Slot) -> &mut [PackedEdge];
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: Slot) -> *mut PackedEdge;
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut PackedEdge;
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [PackedEdge];
    fn set(&mut self, src: VHandle, val: PackedEdge, offset: Slot);
}
pub trait EdgeManipulate: EdgeStore + GraphOperate + EdgeVisit + Clone{}
pub trait WeightedEdgeManipulate: EdgeManipulate + WeightedGraphOperate {}