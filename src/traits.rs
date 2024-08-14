use std::collections::btree_map::Iter;
use std::ops::{Index, IndexMut};
use crate::handles::Slot;
use crate::handles::types::{Edge, EHandle, Weight};

pub trait StoreVertex: Index<EHandle, Output=Self::VertexType> + IndexMut<EHandle, Output=Self::VertexType>{
    type VertexType;
    fn len(&self) -> usize;
    fn push(&mut self, val: Self::VertexType);
    fn capacity(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::VertexType>;
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VertexType>;
    fn as_slice(&self) -> &[Self::VertexType];
}

pub trait EdgeConnect {
    fn connect_edges(&mut self, src: EHandle, targets: &[Edge]);
    fn disconnect(&mut self, src_handle: EHandle, handle: EHandle);
    fn connect(&mut self, from: EHandle, to: EHandle);
}

pub trait WeightedEdgeConnect {
    fn connect_weighted(&mut self, from: EHandle, to: EHandle, weight: Weight);
}

pub trait EdgeStore: Index<Slot, Output=Slot> + IndexMut<Slot, Output=Slot>{
    fn create_entry(&mut self, size: Slot) -> EHandle;
    fn entry_as_slice(&self, handle: EHandle) -> &[Edge];
    fn entry_as_mut_slice(&mut self, handle: EHandle) -> &mut [Edge];
    fn entry_as_ptr(&self, handle: EHandle) -> *const Edge;
    fn entry_as_mut_ptr(&mut self, handle: EHandle) -> *mut Edge;
    fn entry_len(&self, handle: EHandle) -> Slot;
    fn entry_capacity(&self, handle: EHandle) -> Slot;
    fn entry_index(&self, handle: EHandle) -> Slot;
    fn iter (&self) -> impl Iterator<Item=&Slot>;
    fn iter_mut (&mut self) -> impl Iterator<Item=&mut Slot>;
    fn entry_iter(&self, handle: EHandle) -> impl Iterator<Item=&Slot>;
    fn entry_iter_mut(&mut self, handle: EHandle) -> impl Iterator<Item=&mut Slot>;
}
pub trait EdgeManipulate: EdgeStore + EdgeConnect + Clone{}
pub trait WeightedEdgeManipulate: EdgeManipulate + WeightedEdgeConnect {}