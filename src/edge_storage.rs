use std::mem::size_of;
use std::ops::{Index, IndexMut};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use crate::handles::{pack, Slot, vh};
use crate::handles::types::{VHandle, Weight, Edge};
use crate::traits::{EdgeManipulate, GraphOperate, EdgeStore, WeightedEdgeManipulate, WeightedGraphOperate, EdgeStorageIterator};

const LEN_OFFSET: Slot = 0;
const CAPACITY_OFFSET: Slot = 1;
pub const HEADER_SIZE: Slot = 2;
#[repr(C)]
pub struct Header{
    pub len: VHandle,
    pub capacity: VHandle,
}

pub struct EdgeStorage {
    pub(in crate) vertex_capacity: Slot,
    pub edges: Vec<Slot>,
    pub indices: Vec<Slot>, //Todo, make it contain EHandles which are not compatible with VHandles
}

pub struct EdgeStorageIter<'a> {
    edges: &'a Vec<Slot>,
    started: bool,
    index: usize,
    chunk_end: usize,
    next_chunk: usize,
    data_end: usize,
}
impl <'a> EdgeStorageIter<'a>{
    pub fn new(edges: &'a Vec<Slot>) -> Self {
        let start: usize = 0;
        let len = edges[start + LEN_OFFSET as usize] as usize;
        let capacity = edges[ start + CAPACITY_OFFSET as usize] as usize;
        let next_chunk = start + capacity + HEADER_SIZE as usize;
        let chunk_end = start + len + HEADER_SIZE as usize;
        let data_end = edges.len();
        let index = start + HEADER_SIZE as usize;
        EdgeStorageIter {
            edges,
            started: false,
            index,
            chunk_end,
            next_chunk,
            data_end,
        }
    }
}

impl<'a> Iterator for EdgeStorageIter<'a> {
    type Item = &'a Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.data_end {
            return None;
        }

        while self.index == self.chunk_end && self.next_chunk != self.data_end {
            self.index = self.next_chunk;
            let len = self.edges[self.index + LEN_OFFSET as usize] as usize;
            let capacity = self.edges[ self.index + CAPACITY_OFFSET as usize ] as usize;
            self.next_chunk = self.index + capacity + HEADER_SIZE as usize;
            self.chunk_end = self.index + len + HEADER_SIZE as usize;
            self.index += HEADER_SIZE as usize;
        }

        let result = self.edges.get(self.index);
        self.index += 1;
        result
    }
}
impl<'a> EdgeStorageIterator for EdgeStorageIter<'a> {
    type Output = &'a Slot;
    fn edge_index(&self) -> usize {
        self.index
    }
}

pub struct EdgeStorageIterMut<'a> {
    edges: &'a mut Vec<Slot>,
    started: bool,
    index: usize,
    chunk_end: usize,
    next_chunk: usize,
    data_end: usize,
}
impl <'a> EdgeStorageIterMut<'a> {
    pub fn new(edges: &'a mut  Vec<Slot>) -> Self {
        let start: usize = 0;
        let len = edges[start + LEN_OFFSET as usize] as usize;
        let capacity = edges[ start + CAPACITY_OFFSET as usize] as usize;
        let next_chunk = start + capacity + HEADER_SIZE as usize;
        let chunk_end = start + len + HEADER_SIZE as usize;
        let data_end = edges.len();
        let index = start + HEADER_SIZE as usize;
        EdgeStorageIterMut {
            edges,
            started: false,
            index,
            chunk_end,
            next_chunk,
            data_end,
        }
    }
}

impl<'a> Iterator for EdgeStorageIterMut<'a> {
    type Item = &'a mut Slot;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.data_end {
            return None;
        }

        while self.index == self.chunk_end {
            self.index = self.next_chunk;
            let len = self.edges[self.index + LEN_OFFSET as usize] as usize;
            let capacity = self.edges[ self.index + CAPACITY_OFFSET as usize ] as usize;
            self.next_chunk = self.index + capacity + HEADER_SIZE as usize;
            self.chunk_end = self.index + len + HEADER_SIZE as usize;
            self.index += HEADER_SIZE as usize;
        }

        let result = Some(unsafe{self.edges.as_mut_ptr().add(self.index).as_mut().unwrap()});
        self.index += 1;
        result
    }
}
impl<'a> EdgeStorageIterator for EdgeStorageIterMut<'a> {
    type Output = &'a mut Slot;
    fn edge_index(&self) -> usize {
        self.index
    }
}

impl Header {
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr_mut (edges: &mut Vec<Slot>, index: usize) -> (*mut Self, *mut VHandle) {
        let edges_ptr = edges.as_mut_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut VHandle;
            (header_ptr, data_ptr)
        }
    }
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr (edges: &Vec<Slot>, index: usize) -> (*const Self, *const VHandle) {
        let edges_ptr = edges.as_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const VHandle;
            (header_ptr, data_ptr)
        }
    }
    pub fn parse_mut (edges: &mut Vec<Slot>, index: usize) -> (&mut Self, &mut [VHandle]) {

        let edges_ptr = edges.as_mut_ptr();

        // Return as Result instead of panic
        if index >= edges.len() {
            panic!("Index out of bounds");
        }

        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut VHandle;
            let data = from_raw_parts_mut(data_ptr, (*header_ptr).capacity as usize);
            return (header_ptr.as_mut().unwrap(), data);
        }
    }
    pub fn parse (edges: &Vec<Slot>, index: usize) -> (&Self, &[VHandle]) {

        // Return as Result instead of panic
        let edges_ptr = edges.as_ptr();

        if index >= edges.len() {
            panic!("Index out of bounds");
        }
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const VHandle;
            let data = from_raw_parts(data_ptr, (*header_ptr).len as usize);
            return (header_ptr.as_ref().unwrap(), data);
        }
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
            vertex_capacity: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: Slot) -> Self {
        EdgeStorage {
            vertex_capacity: capacity,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. No reserve.
    pub fn new() -> Self {
        EdgeStorage {
            vertex_capacity: 0,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn len_mut_ptr(&mut self, vertex: VHandle) -> *mut Slot {
        &mut self.edges[ (self.indices[vertex as usize] + LEN_OFFSET) as usize]
    }

    #[allow(unused)]
    fn reserve_mut(&mut self, vertex: VHandle) -> &mut Slot {
        let edge_chunk_index = self.indices[vertex as usize];
        &mut self.edges[ (edge_chunk_index + CAPACITY_OFFSET) as usize]
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len_mut(&mut self, vertex: VHandle) -> &mut Slot {
        let edge_chunk_index = self.indices[vertex as usize];
        &mut self.edges[ (edge_chunk_index + LEN_OFFSET) as usize]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn calculate_new_edges_size_abs(&self, size: Slot) -> Slot {
        let header_size = HEADER_SIZE;
        (self.edges.len() as Slot + self.vertex_capacity + header_size + size) as Slot
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn capacity(&self) -> Slot {
        self.edges.len() as Slot
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
        *self.len_mut(from) = new_size as Slot;
    }

    fn create_edges_entry(&mut self, size: Slot) -> VHandle {
        let offset = self.edges.len() as Slot;
        let val = self.calculate_new_edges_size_abs(size);
        self.edges.resize_with(val as usize, Default::default);
        self.edges[ (offset + CAPACITY_OFFSET) as usize] = self.vertex_capacity + size;
        self.indices.push(offset);
        (self.indices.len() - 1) as VHandle
    }

    fn disconnect(&mut self, from: VHandle, to: VHandle) {
        let data = self.edges_mut_ptr(from);
        let len = self.len_mut_ptr(from);
        unsafe {
            let mut iter = data;
            let end = iter.add(*len as usize);
            while iter != end{
                if vh(*iter) == to {
                    *iter = *end.offset(-1 ); // Swap the last element for the empty one
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

impl WeightedGraphOperate for EdgeStorage{
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight) {
        self.add_edges(from, &[pack(to, weight)]);
    }
}
impl EdgeStore for EdgeStorage {
    fn edges_offset(&self, vertex: VHandle, offset: Slot) -> &[Edge] {

        let edge_chunk_index = self.indices[vertex as usize];
        let len = self.edges[ (edge_chunk_index + LEN_OFFSET) as usize];
        
        &self.edges[ (offset + edge_chunk_index + HEADER_SIZE) as usize.. (edge_chunk_index + HEADER_SIZE + len) as usize]
    }
    fn edges_ptr_offset(&self, vertex: VHandle, offset: Slot) -> *const Edge {

        let edge_chunk_index = self.indices[vertex as usize];
        unsafe {self.edges.as_ptr().add((offset + edge_chunk_index + HEADER_SIZE) as usize)}
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges(&self, vertex: VHandle) -> &[Edge] {
        return self.edges_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_ptr(&self, vertex: VHandle) -> *const Edge {
        self.edges_ptr_offset(vertex, 0)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len(&self, handle: VHandle) -> Slot {
        self.edges[ (self.indices[handle as usize] + LEN_OFFSET) as usize]
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edge_block_capacity(&self, handle: VHandle) -> Slot {
        self.edges[ (self.indices[handle as usize] + CAPACITY_OFFSET) as usize]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get_edges_index(&self, vertex: VHandle) -> Slot {
        
        self.indices[vertex as usize] + HEADER_SIZE
    }

    fn edges_mut_offset(&mut self, vertex: VHandle, offset: Slot) -> &mut [Edge] {

        let edge_chunk_index = self.indices[vertex as usize];
        let reserve = self.edges[ (edge_chunk_index + CAPACITY_OFFSET) as usize];
        
        (&mut self.edges[ (offset + edge_chunk_index + HEADER_SIZE) as usize..(edge_chunk_index + HEADER_SIZE + reserve) as usize]) as _
    }
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: Slot) -> *mut Edge {

        let edge_chunk_index = self.indices[vertex as usize];
        unsafe {self.edges.as_mut_ptr().add((offset + edge_chunk_index + HEADER_SIZE) as usize)}
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut Edge {
        self.edges_mut_ptr_offset(vertex, 0)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [Edge] {
        return self.edges_mut_offset(vertex, 0);
    }
    fn iter(&self) -> impl EdgeStorageIterator<Output=&Slot> {
        return EdgeStorageIter::new(&self.edges);
    }

    fn iter_mut(&mut self) -> impl EdgeStorageIterator<Output=&mut Slot> {
        return EdgeStorageIterMut::new(&mut self.edges);
    }
}
impl Clone for EdgeStorage {
    fn clone(&self) -> Self {
        EdgeStorage {
            vertex_capacity: self.vertex_capacity,
            edges: self.edges.clone(),
            indices: self.indices.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertex_capacity = source.vertex_capacity;
        self.edges.clone_from(&source.edges);
        self.indices.clone_from(&source.indices);
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