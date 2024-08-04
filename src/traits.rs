use std::collections::btree_set::Iter;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use crate::handles::Slot;
use crate::handles::types::{PackedEdge, VHandle, Weight};

// TODO add iters for edges
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

pub trait EdgeStorageIterator: Iterator<Item=Self::Output>{
    type Output;
    fn edge_index(&self) -> usize;
    fn enumerate_as_index(&mut self) -> Option<(usize, Self::Output)> {
        let index = self.edge_index();
        match self.next() {
            None => None,
            Some(next) => Some((index, next))
        }
    }
}

pub trait EdgeStore{
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
    fn iter (&self) -> impl EdgeStorageIterator<Output=&Slot>;
    fn iter_mut (&mut self) -> impl EdgeStorageIterator<Output=&mut Slot>;
}
pub trait EdgeManipulate: EdgeStore + GraphOperate + Clone{}
pub trait WeightedEdgeManipulate: EdgeManipulate + WeightedGraphOperate {}