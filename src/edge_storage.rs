use std::mem::size_of;
use std::ops::{Index, IndexMut};
use std::path::Iter;
use std::ptr::{null, null_mut};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use crate::handles::{pack, Slot, vh};
use crate::handles::types::{VHandle, Weight, Edge};
use crate::traits::{EdgeManipulate, GraphOperate, EdgeStore, WeightedEdgeManipulate, WeightedGraphOperate};
#[derive(Copy, Clone)]
pub struct EdgesEntry {
    pub len: Slot,
    pub capacity: Slot,
    pub chunk_offset: Slot,
}

pub struct EdgeStorage {
    pub(in crate) reserve: Slot,
    pub edges: Vec<Edge>,
    edges_entries: Vec<EdgesEntry>,
}

pub struct EdgeStorageIter<'a> {
    edges: &'a Vec<Edge>,
    current: usize,
    len: usize,
    entries_iter: core::slice::Iter<'a, EdgesEntry>,
}
impl<'a> EdgeStorageIter<'a> {
    pub fn new(edge_storage: &'a EdgeStorage) -> Self {
        EdgeStorageIter {
            edges: &edge_storage.edges,
            current: 0,
            len: 0,
            entries_iter: edge_storage.edges_entries.iter(),
        }
    }
}

impl<'a> Iterator for EdgeStorageIter<'a> {
    type Item = &'a Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.len {
            let result = self.entries_iter.next();
            if result.is_none() {
                return None;
            }
            let next = result.unwrap();
            self.current = next.chunk_offset as usize;
            self.len = (next.chunk_offset + next.len) as usize;
        }
        let result = self.edges.get(self.current);
        self.current += 1;
        result
    }
}


pub struct EdgeStorageIterMut<'a> {
    edges: &'a mut Vec<Edge>,
    current: usize,
    len: usize,
    entries_iter: core::slice::IterMut<'a, EdgesEntry>,
}

impl<'a> EdgeStorageIterMut<'a> {
    pub fn new(edge_storage: &'a mut EdgeStorage) -> Self {
        EdgeStorageIterMut {
            edges: &mut edge_storage.edges,
            current: 0,
            len: 0,
            entries_iter: edge_storage.edges_entries.iter_mut(),
        }
    }
}
impl<'a> Iterator for EdgeStorageIterMut<'a> {
    type Item = &'a mut Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.len {
            let result = self.entries_iter.next();
            if result.is_none() {
                return None;
            }
            let next = result.unwrap();
            self.current = next.chunk_offset as usize;
            self.len = (next.chunk_offset + next.len) as usize;
        }
        let result = unsafe{ (&mut self.edges[self.current] as *mut Edge).as_mut() };
        self.current += 1;
        result
    }
}


pub struct EdgeIterator<'a> {
    edge_storage: &'a EdgeStorage,
    index: usize,
    len: usize,
}

impl<'a> EdgeIterator<'a> {
    pub fn new(edge_storage: &'a EdgeStorage, handle: VHandle) -> Self {
        let index = edge_storage.get_edges_index(handle) as usize;
        let len = index + edge_storage.len(handle) as usize;
        EdgeIterator {
            edge_storage,
            index,
            len,
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }
        let result = self.edge_storage.edges.get(self.index);
        self.index += 1;
        result
    }
}


pub struct EdgeIteratorMut<'a> {
    edge_storage: &'a mut EdgeStorage,
    index: usize,
    len: usize,
}

impl<'a> EdgeIteratorMut<'a> {
    #[inline(always)]
    pub fn new(edge_storage: &'a mut EdgeStorage, handle: VHandle) -> Self {
        let index = edge_storage.get_edges_index(handle) as usize;
        let len = index + edge_storage.len(handle) as usize;
        EdgeIteratorMut {
            edge_storage,
            index,
            len,
        }
    }
}

impl<'a> Iterator for EdgeIteratorMut<'a> {
    type Item = &'a mut Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }
        let result = unsafe{Some((&mut self.edge_storage[self.index as Slot] as *mut Slot).as_mut().unwrap())};
        self.index += 1;
        result
    }
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
            edges: Vec::new(),
            edges_entries: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: Slot) -> Self {
        EdgeStorage {
            reserve: capacity,
            edges: Vec::new(),
            edges_entries: Vec::new(),
        }
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. No reserve.
    pub fn new() -> Self {
        EdgeStorage {
            reserve: 0,
            edges: Vec::new(),
            edges_entries: Vec::new(),
        }
    }
}

impl GraphOperate for EdgeStorage {
    fn add_edges(&mut self, from: VHandle, to: &[Edge]) {
        let len = self.len(from) as usize;
        let new_size = len + to.len();

        if new_size > self.edge_block_capacity(from) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.edges_mut(from);
        data[len..new_size].copy_from_slice(to);
        self.edges_entries[from as usize].len = new_size as Slot;
    }

    fn create_edges_entry(&mut self, size: Slot) -> VHandle {
        let offset = self.edges.len() as Slot;
        self.edges.resize_with((self.edges.len() as Slot + size + self.reserve) as usize, Default::default);
        self.edges_entries.push(EdgesEntry {
            len: 0,
            capacity: self.reserve + size,
            chunk_offset: offset,
        });
        (self.edges_entries.len() - 1) as VHandle
    }

    fn disconnect(&mut self, from: VHandle, to: VHandle) {
        let data = self.edges_mut_ptr(from);
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
        self.add_edges(from, &[pack(to, 0)]);
    }
}

impl WeightedGraphOperate for EdgeStorage {
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight) {
        self.add_edges(from, &[pack(to, weight)]);
    }
}
impl EdgeStore for EdgeStorage {

    fn edges_offset(&self, vertex: VHandle, offset: Slot) -> &[Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        &self.edges[(offset + edge_chunk_meta.chunk_offset) as usize..(edge_chunk_meta.chunk_offset + edge_chunk_meta.len) as usize]
    }
    fn edges_ptr_offset(&self, vertex: VHandle, offset: Slot) -> *const Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_ptr().add((offset + edge_chunk_meta.chunk_offset) as usize) }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges(&self, vertex: VHandle) -> &[Edge] {
        self.edges_offset(vertex, 0)
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_ptr(&self, vertex: VHandle) -> *const Edge {
        self.edges_ptr_offset(vertex, 0)
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len(&self, handle: VHandle) -> Slot {
        self.edges_entries[handle as usize].len as Slot
    }

    fn edge_block_capacity(&self, handle: VHandle) -> Slot {
        self.edges_entries[handle as usize].capacity as Slot
    }

    fn get_edges_index(&self, vertex: VHandle) -> Slot {
        self.edges_entries[vertex as usize].chunk_offset as Slot
    }

    fn edges_mut_offset(&mut self, vertex: VHandle, offset: Slot) -> &mut [Edge] {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        (&mut self.edges[ (edge_chunk_meta.chunk_offset + offset) as usize..(edge_chunk_meta.chunk_offset + edge_chunk_meta.capacity) as usize]) as _
    }
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: Slot) -> *mut Edge {
        let edge_chunk_meta = self.edges_entries[vertex as usize];
        unsafe { self.edges.as_mut_ptr().add((offset + edge_chunk_meta.chunk_offset) as usize) }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut Edge {
        self.edges_mut_ptr_offset(vertex, 0)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [Edge] {
        return self.edges_mut_offset(vertex, 0);
    }
    fn iter(&self) -> impl Iterator<Item=&Slot> {
        EdgeStorageIter::new(self)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut Slot> {
        EdgeStorageIterMut::new(self)
    }

    fn edge_iter(&self, handle: VHandle) -> impl Iterator<Item=&Slot> {
        EdgeIterator::new(self, handle)
    }

    #[inline(always)]
    fn edge_iter_mut(&mut self, handle: VHandle) -> impl Iterator<Item=&mut Slot> {
        EdgeIteratorMut::new(self, handle)
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