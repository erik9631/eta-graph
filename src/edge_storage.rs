use std::ops::{Index, IndexMut};
use eta_algorithms::data_structs::array::Array;
use crate::handles::{pack, Slot, eh};
use crate::handles::types::{EHandle, Weight, Edge};
use crate::traits::{EdgeManipulate, EdgeConnect, EdgeStore, WeightedEdgeManipulate, WeightedEdgeConnect};
#[derive(Copy, Clone)]
pub struct EdgesEntry {
    pub len: Slot,
    pub capacity: Slot,
    pub chunk_offset: Slot,
}

pub struct EdgeStorageIter<'a> {
    edges: &'a Array<Edge>,
    current: *const Edge,
    end: *const Edge,
    entries_iter: core::slice::Iter<'a, EdgesEntry>,
}
impl<'a> EdgeStorageIter<'a> {
    pub fn new(edge_storage: &'a EdgeStorage) -> Self {
        let mut entries_iter = edge_storage.edges_entries.iter();
        let next = entries_iter.next().unwrap();
        let current = unsafe { edge_storage.edges.as_ptr().add(next.chunk_offset as usize) };
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
            type Item = &'a $($mut_type)? Slot;
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
        $self.current = unsafe { $self.edges.as_ptr().add($next.chunk_offset as usize) };
    };
    (@get_current $self:ident ,$next:ident mut) => {
        $self.current = unsafe { $self.edges.as_mut_ptr().add($next.chunk_offset as usize) };
    };
}
edge_storage_iter_impl!(EdgeStorageIter);
pub struct EdgeStorageIterMut<'a> {
    edges: &'a mut Array<Edge>,
    current: *mut Edge,
    end: *mut Edge,
    entries_iter: core::slice::Iter<'a, EdgesEntry>,
}
impl<'a> EdgeStorageIterMut<'a> {
    pub fn new(edge_storage: & 'a mut EdgeStorage) -> Self {
        let mut entries_iter = edge_storage.edges_entries.iter();
        let next = entries_iter.next().unwrap();
        let current = unsafe { edge_storage.edges.as_mut_ptr().add(next.chunk_offset as usize) };
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
    pub(in crate) reserve: Slot,
    pub edges: Array<Edge>,
    edges_entries: Vec<EdgesEntry>,
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
    pub fn with_reserve(capacity: Slot) -> Self {
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
    fn connect_edges(&mut self, from: EHandle, to: &[Edge]) {
        let len = self.entry_len(from) as usize;
        let new_size = len + to.len();

        if new_size > self.entry_capacity(from) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.entry_as_mut_slice(from);
        data[len..new_size].copy_from_slice(to);
        self.edges_entries[from as usize].len = new_size as Slot;
    }

    fn disconnect(&mut self, from: EHandle, to: EHandle) {
        let data = self.entry_as_mut_ptr(from);
        let len = &mut self.edges_entries[from as usize].len;
        unsafe {
            let mut iter = data;
            let end = iter.add(*len as usize);
            while iter != end {
                if eh(*iter) == to {
                    *iter = *end.offset(-1); // Swap the last element for the empty one
                    *len -= 1;
                    break;
                }
                iter = iter.offset(1);
            }
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn connect(&mut self, from: EHandle, to: EHandle) {
        self.connect_edges(from, &[pack(to, 0)]);
    }
}

impl WeightedEdgeConnect for EdgeStorage {
    fn connect_weighted(&mut self, from: EHandle, to: EHandle, weight: Weight) {
        self.connect_edges(from, &[pack(to, weight)]);
    }
}
impl EdgeStore for EdgeStorage {
    fn create_entry(&mut self, size: Slot) -> EHandle {
        let offset = self.edges.capacity() as Slot;
        self.edges.extend_by((size + self.reserve) as usize);
        self.edges_entries.push(EdgesEntry {
            len: 0,
            capacity: self.reserve + size,
            chunk_offset: offset,
        });
        (self.edges_entries.len() - 1) as EHandle
    }
    #[inline(always)]
    fn entry_as_slice(&self, vertex: EHandle) -> &[Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        &self.edges.as_slice()[edge_chunk_meta.chunk_offset as usize..(edge_chunk_meta.chunk_offset + edge_chunk_meta.len) as usize]
    }

    #[inline(always)]
    fn entry_as_mut_slice(&mut self, vertex: EHandle) -> &mut [Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        &mut self.edges.as_slice_mut()[ edge_chunk_meta.chunk_offset as usize..(edge_chunk_meta.chunk_offset + edge_chunk_meta.capacity) as usize]
    }

    #[inline(always)]
    fn entry_as_ptr(&self, vertex: EHandle) -> *const Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_ptr().add(edge_chunk_meta.chunk_offset as usize) }
    }

    #[inline(always)]
    fn entry_as_mut_ptr(&mut self, vertex: EHandle) -> *mut Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_mut_ptr().add(edge_chunk_meta.chunk_offset as usize) }
    }

    #[inline(always)]
    fn entry_len(&self, handle: EHandle) -> Slot {
        self.edges_entries[handle as usize].len as Slot
    }

    fn entry_capacity(&self, handle: EHandle) -> Slot {
        self.edges_entries[handle as usize].capacity as Slot
    }

    fn entry_index(&self, vertex: EHandle) -> Slot {
        self.edges_entries[vertex as usize].chunk_offset as Slot
    }

    fn iter(&self) -> impl Iterator<Item=&Slot> {
        EdgeStorageIter::new(self)
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut Slot> {
        EdgeStorageIterMut::new(self)
    }

    #[inline(always)]
    fn entry_iter(&self, handle: EHandle) -> impl Iterator<Item=&Slot> {
        self.edges.iter_range(self.entry_index(handle) as usize, self.entry_len(handle) as usize)
    }

    #[inline(always)]
    fn entry_iter_mut(&mut self, handle: EHandle) -> impl Iterator<Item=&mut Slot> {
        self.edges.iter_range_mut(self.entry_index(handle) as usize, self.entry_len(handle) as usize)
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

impl Index<Slot> for EdgeStorage {
    type Output = Slot;
    fn index(&self, index: Slot) -> &Self::Output {
        &self.edges[index as usize]
    }
}

impl IndexMut<Slot> for EdgeStorage {
    fn index_mut(&mut self, index: Slot) -> &mut Self::Output {
        &mut self.edges[index as usize]
    }
}

impl EdgeManipulate for EdgeStorage {}

impl WeightedEdgeManipulate for EdgeStorage {}