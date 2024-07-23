use crate::size::{VHandle};

pub trait Transformer<T>{
    fn transform(&mut self, transform_fn: fn(&mut [T]));
    fn async_transform(&mut self, transform_fn: fn(&mut [T]));
}

pub trait EdgeOperator {
    fn add_edges(&mut self, src: VHandle, targets: &[VHandle]);
    fn extend_edge_storage(&mut self, size: usize) -> VHandle;
    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle);
    fn connect(&mut self, from: VHandle, to: VHandle);
}

pub trait TraverseMarker {
    fn global_visited_flag(&self) -> VHandle;
    fn inc_global_visited_flag(&mut self);
    fn reset_global_visited_flag(&mut self);
    fn visited_flag(&self, vertex: VHandle) -> VHandle;
    fn inc_visited_flag(&mut self, vertex: VHandle);
    fn set_visited_flag(&mut self, vertex: VHandle, val: VHandle);
}
pub trait EdgeStore {
    fn edges_offset(&self, vertex: VHandle, offset: usize) -> &[VHandle];
    fn edges_ptr_offset(&self, vertex: VHandle, offset: usize) -> *const VHandle;
    fn edges(&self, vertex: VHandle) -> &[VHandle];
    fn edges_ptr(&self, vertex: VHandle) -> *const VHandle;
    fn len(&self, handle: VHandle) -> VHandle;
    fn edge_block_capacity(&self, handle: VHandle) -> usize;
    fn get(&self, vertex: VHandle, offset: usize) -> VHandle;
}

pub trait EdgeStoreMut: EdgeStore {
    fn edges_mut_offset(&mut self, vertex: VHandle, offset: usize) -> &mut [VHandle];
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: usize) -> *mut VHandle;
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut VHandle;
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [VHandle];
    fn set(&mut self, src: VHandle, val: VHandle, offset: usize);
}