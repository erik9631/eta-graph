use std::ops::{Index, IndexMut};
use crate::handles::types::{Edge, VHandle, Weight, Ci};

pub trait StoreVertex: Index<VHandle, Output=Self::VertexType> + IndexMut<VHandle, Output=Self::VertexType>{
    type VertexType;
    fn len(&self) -> usize;
    fn push(&mut self, val: Self::VertexType);
    fn capacity(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::VertexType>;
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VertexType>;
    fn as_slice(&self) -> &[Self::VertexType];
}

pub trait EdgeConnect {
    fn connect_edges(&mut self, src: VHandle, targets: &[Edge]);
    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle);
    fn connect(&mut self, from: VHandle, to: VHandle);
}

pub trait WeightedEdgeConnect {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight);
}

pub trait EdgeStore: Index<usize, Output=Edge> + IndexMut<usize, Output=Edge>{
    fn create_vertex_entry(&mut self, size: Ci) -> VHandle;
    fn edges_as_slice(&self, handle: VHandle) -> &[Edge];
    fn edges_as_slice_mut(&mut self, handle: VHandle) -> &mut [Edge];
    fn edges_as_ptr(&self, handle: VHandle) -> *const Edge;
    fn edges_as_mut_ptr(&mut self, handle: VHandle) -> *mut Edge;
    fn edges_len(&self, handle: VHandle) -> usize;
    fn edges_capacity(&self, handle: VHandle) -> usize;
    fn edges_index(&self, handle: VHandle) -> usize;
    fn iter(&self) -> impl Iterator<Item=&Edge>;
    fn iter_mut (&mut self) -> impl Iterator<Item=&mut Edge>;
    fn edges_iter(&self, handle: VHandle) -> impl Iterator<Item=&Edge>;
    fn edges_iter_mut(&mut self, handle: VHandle) -> impl Iterator<Item=&mut Edge>;
}
pub trait EdgeManipulate: EdgeStore + EdgeConnect + Clone{}
pub trait WeightedEdgeManipulate: EdgeManipulate + WeightedEdgeConnect {}