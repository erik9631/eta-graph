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

pub trait StoreVertex<T>: Index<VHandle> + IndexMut<VHandle> + Clone{
    type Item;
    fn len(&self) -> usize;
    fn push(&mut self, val: Self::Item);
    fn capacity(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::Item>;
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::Item>;
    fn as_slice(&self) -> &[Self::Item];
}

pub trait Operate {
    fn add_edges(&mut self, src: VHandle, targets: &[PackedEdge]);
    fn extend_edge_storage(&mut self, size: Slot) -> Slot;
    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle);
    fn connect(&mut self, from: VHandle, to: VHandle);
}

pub trait WeightedOperate {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight);
}

pub trait Visit {
    fn global_visited_flag(&self) -> Slot;
    fn inc_global_visited_flag(&mut self);
    fn reset_global_visited_flag(&mut self);
    fn visited_flag(&self, vertex: VHandle) -> Slot;
    fn inc_visited_flag(&mut self, vertex: VHandle);
    fn set_visited_flag(&mut self, vertex: VHandle, val: Slot);
}
pub trait Store {
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
pub trait Manipulate: Store + Operate + Visit + Clone{}
pub trait WeightedManipulate: Manipulate + WeightedOperate {}