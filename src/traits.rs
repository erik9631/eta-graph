use std::collections::btree_map::Iter;
use std::ops::{Index, IndexMut};
use crate::handles::Slot;
use crate::handles::types::{Edge, VHandle, Weight};

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
    fn add_edges(&mut self, src: VHandle, targets: &[Edge]);
    fn create_edges_entry(&mut self, size: Slot) -> VHandle;
    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle);
    fn connect(&mut self, from: VHandle, to: VHandle);
}

pub trait WeightedGraphOperate {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight);
}
pub trait EdgeStore: Index<Slot, Output=Slot> + IndexMut<Slot, Output=Slot>{
    fn edges_offset(&self, vertex: VHandle, offset: Slot) -> &[Edge];
    fn edges_ptr_offset(&self, vertex: VHandle, offset: Slot) -> *const Edge;
    fn edges(&self, vertex: VHandle) -> &[Edge];
    fn edges_ptr(&self, vertex: VHandle) -> *const Edge;
    fn len(&self, handle: VHandle) -> Slot;
    fn edge_block_capacity(&self, handle: VHandle) -> Slot;
    fn get_edges_index(&self, vertex: VHandle) -> Slot;
    fn edges_mut_offset(&mut self, vertex: VHandle, offset: Slot) -> &mut [Edge];
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: Slot) -> *mut Edge;
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut Edge;
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [Edge];
    fn iter (&self) -> impl Iterator<Item=&Slot>;
    fn iter_mut (&mut self) -> impl Iterator<Item=&mut Slot>;
}
pub trait EdgeManipulate: EdgeStore + GraphOperate + Clone{}
pub trait WeightedEdgeManipulate: EdgeManipulate + WeightedGraphOperate {}