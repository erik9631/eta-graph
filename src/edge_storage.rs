use std::ops::{Index, IndexMut};
use eta_algorithms::data_structs::array::Array;
use crate::handles::{pack, vh};
use crate::handles::types::{VHandle, Weight, Edge, Ci};
use crate::traits::{EdgeManipulate, EdgeConnect, EdgeStore, WeightedEdgeManipulate, WeightedEdgeConnect};
#[derive(Copy, Clone)]
pub struct Vertex {
    pub len: Ci,
    pub capacity: Ci,
    pub offset: Ci,
}

pub struct EdgeStorageIter<'a> {
    edges: &'a Array<Edge>,
    current: *const Edge,
    end: *const Edge,
    entries_iter: core::slice::Iter<'a, Vertex>,
}
impl<'a> EdgeStorageIter<'a> {
    pub fn new(edge_storage: &'a EdgeStorage) -> Self {
        let mut entries_iter = edge_storage.edges_entries.iter();
        let next = entries_iter.next().unwrap();
        let current = unsafe { edge_storage.edges.as_ptr().add(next.offset as usize) };
        let end = unsafe { current.add(next.len as usize) };
        EdgeStorageIter {
            edges: &edge_storage.edges,
            current,
            end,
            entries_iter,
        }
    }
}

macro_rules! edge_storage_iter_impl {
    ($impl_name:ident $(,$mut_type:ident)?) => {
        impl<'a> Iterator for $impl_name<'a> {
            type Item = &'a $($mut_type)? Edge;
            fn next(&mut self) -> Option<Self::Item> {
                while self.current == self.end {
                    let result = self.entries_iter.next();
                    if result.is_none() {
                        return None;
                    }
                    let next = result.unwrap();
                    edge_storage_iter_impl!(@get_current self, next $($mut_type)?);

                    self.end = unsafe { self.current.add(next.len as usize) };
                }
                let result = edge_storage_iter_impl!(@get_result self, next $($mut_type)?);
                self.current = unsafe { self.current.add(1) };
                result
            }
        }
    };

    (@get_result $self:ident ,$next:ident) => {
        unsafe{Some($self.current.as_ref().unwrap())}
    };

    (@get_result $self:ident ,$next:ident mut) => {
        unsafe{Some($self.current.as_mut().unwrap())}
    };

    (@get_current $self:ident ,$next:ident) => {
        $self.current = unsafe { $self.edges.as_ptr().add($next.offset as usize) };
    };
    (@get_current $self:ident ,$next:ident mut) => {
        $self.current = unsafe { $self.edges.as_mut_ptr().add($next.offset as usize) };
    };
}
edge_storage_iter_impl!(EdgeStorageIter);
pub struct EdgeStorageIterMut<'a> {
    edges: &'a mut Array<Edge>,
    current: *mut Edge,
    end: *mut Edge,
    entries_iter: core::slice::Iter<'a, Vertex>,
}
impl<'a> EdgeStorageIterMut<'a> {
    pub fn new(edge_storage: & 'a mut EdgeStorage) -> Self {
        let mut entries_iter = edge_storage.edges_entries.iter();
        let next = entries_iter.next().unwrap();
        let current = unsafe { edge_storage.edges.as_mut_ptr().add(next.offset as usize) };
        let end = unsafe { current.add(next.len as usize) };
        EdgeStorageIterMut {
            edges: &mut edge_storage.edges,
            current,
            end,
            entries_iter,
        }
    }
}
edge_storage_iter_impl!(EdgeStorageIterMut, mut);

pub struct EdgeStorage {
    pub(in crate) reserve: Ci,
    pub edges: Array<Edge>,
    edges_entries: Vec<Vertex>,
}

impl Default for EdgeStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl EdgeStorage {
    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_large() -> Self {
        EdgeStorage {
            reserve: 50,
            edges: Array::new(0),
            edges_entries: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: Ci) -> Self {
        EdgeStorage {
            reserve: capacity,
            edges: Array::new(0),
            edges_entries: Vec::new(),
        }
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. No reserve.
    pub fn new() -> Self {
        EdgeStorage {
            reserve: 0,
            edges: Array::new(0),
            edges_entries: Vec::new(),
        }
    }
}

impl EdgeConnect for EdgeStorage {
    fn connect_edges(&mut self, from: VHandle, to: &[Edge]) {
        let len = self.vertex_len(from) as usize;
        let new_size = len + to.len();

        if new_size > self.vertex_capacity(from) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.vertex_as_slice_mut(from);
        data[len..new_size].copy_from_slice(to);
        self.edges_entries[from as usize].len = new_size as Ci;
    }

    fn disconnect(&mut self, from: VHandle, to: VHandle) {
        let data = self.vertex_as_mut_ptr(from);
        let len = &mut self.edges_entries[from as usize].len;
        unsafe {
            let mut iter = data;
            let end = iter.add(*len as usize);
            while iter != end {
                if vh(*iter) == to {
                    *iter = *end.offset(-1); // Swap the last element for the empty one
                    *len -= 1;
                    break;
                }
                iter = iter.offset(1);
            }
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn connect(&mut self, from: VHandle, to: VHandle) {
        self.connect_edges(from, &[pack(to, 0)]);
    }
}

impl WeightedEdgeConnect for EdgeStorage {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight) {
        self.connect_edges(from, &[pack(to, weight)]);
    }
}
impl EdgeStore for EdgeStorage {
    fn create_vertex(&mut self, size: Ci) -> VHandle {
        let offset = self.edges.capacity() as Ci;
        self.edges.extend_by((size + self.reserve) as usize);
        self.edges_entries.push(Vertex {
            len: 0,
            capacity: self.reserve + size,
            offset: offset,
        });
        (self.edges_entries.len() - 1) as VHandle
    }
    #[inline(always)]
    fn vertex_as_slice(&self, vertex: VHandle) -> &[Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        &self.edges.as_slice()[edge_chunk_meta.offset as usize..(edge_chunk_meta.offset + edge_chunk_meta.len) as usize]
    }

    #[inline(always)]
    fn vertex_as_slice_mut(&mut self, vertex: VHandle) -> &mut [Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        &mut self.edges.as_slice_mut()[ edge_chunk_meta.offset as usize..(edge_chunk_meta.offset + edge_chunk_meta.capacity) as usize]
    }

    #[inline(always)]
    fn vertex_as_ptr(&self, vertex: VHandle) -> *const Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_ptr().add(edge_chunk_meta.offset as usize) }
    }

    #[inline(always)]
    fn vertex_as_mut_ptr(&mut self, vertex: VHandle) -> *mut Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_mut_ptr().add(edge_chunk_meta.offset as usize) }
    }

    #[inline(always)]
    fn vertex_len(&self, handle: VHandle) -> usize {
        self.edges_entries[handle as usize].len as usize
    }

    fn vertex_capacity(&self, handle: VHandle) -> usize {
        self.edges_entries[handle as usize].capacity as usize
    }

    fn vertex_index(&self, vertex: VHandle) -> usize {
        self.edges_entries[vertex as usize].offset as usize
    }

    fn iter(&self) -> impl Iterator<Item=&Edge> {
        EdgeStorageIter::new(self)
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut Edge> {
        EdgeStorageIterMut::new(self)
    }

    #[inline(always)]
    fn vertex_iter(&self, handle: VHandle) -> impl Iterator<Item=&Edge> {
        self.edges.iter_range(self.vertex_index(handle), self.vertex_len(handle))
    }

    #[inline(always)]
    fn vertex_iter_mut(&mut self, handle: VHandle) -> impl Iterator<Item=&mut Edge> {
        self.edges.iter_range_mut(self.vertex_index(handle), self.vertex_len(handle))
    }
}
impl Clone for EdgeStorage {
    fn clone(&self) -> Self {
        EdgeStorage {
            reserve: self.reserve,
            edges: self.edges.clone(),
            edges_entries: self.edges_entries.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.reserve = source.reserve;
        self.edges.clone_from(&source.edges);
        self.edges_entries.clone_from(&source.edges_entries);
    }
}

impl Index<usize> for EdgeStorage {
    type Output = Edge;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl IndexMut<usize> for EdgeStorage {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl EdgeManipulate for EdgeStorage {}

impl WeightedEdgeManipulate for EdgeStorage {}