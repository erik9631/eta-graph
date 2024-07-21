use crate::size::MSize;

pub trait Transform<T>{
    fn transform(&mut self, transform_fn: fn(&mut [T]));
    fn async_transform(&mut self, transform_fn: fn(&mut [T]));
}

pub trait TraverseMarker {
    fn global_visited_flag(&self) -> MSize;
    fn inc_global_visited_flag(&mut self);
    fn reset_global_visited_flag(&mut self);
    fn visited_flag(&self, vertex: MSize) -> MSize;
    fn inc_visited_flag(&mut self, vertex: MSize);
    fn set_visited_flag(&mut self, vertex: MSize, val: MSize);
}
pub trait GraphAccessor {
    fn edges_offset(&self, vertex: MSize, offset: usize) -> &[MSize];
    fn edges_ptr_offset(&self, vertex: MSize, offset: usize) -> *const MSize;
    fn edges(&self, vertex: MSize) -> &[MSize];
    fn edges_ptr(&self, vertex: MSize) -> *const MSize;
    fn len(&self, handle: MSize) -> MSize;
    fn vertex_capacity(&self, handle: MSize) -> MSize;
}

pub trait GraphAccessorMut: GraphAccessor {
    fn edges_mut_offset(&mut self, vertex: MSize, offset: usize) -> &mut [MSize];
    fn edges_mut_ptr_offset(&mut self, vertex: MSize, offset: usize) -> *mut MSize;
    fn edges_mut_ptr(&mut self, vertex: MSize) -> *mut MSize;
    fn edges_mut(&mut self, vertex: MSize) -> &mut [MSize];
}