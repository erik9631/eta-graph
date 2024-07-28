use std::mem::size_of;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use firestorm::{profile_method, profile_section};
use crate::graph::{Error};
use crate::handles::{pack, Slot, vh};
use crate::handles::types::{VHandle, Weight, PackedEdge};
use crate::traits::{Manipulate, Operate, Store, Visit, WeightedManipulate, WeightedOperate};

const FLAG_OFFSET: Slot = 0;
const LEN_OFFSET: Slot = 1;
const CAPACITY_OFFSET: Slot = 2;
pub const HEADER_SIZE: Slot = 3;
#[repr(C)]
pub struct Header{
    pub visited_flag: VHandle,
    pub len: VHandle,
    pub capacity: VHandle,
}

pub struct EdgeStorage {
    pub (in crate) global_visited_flag: Slot, // Val used to mark whether the vertex has been visited
    pub(in crate) vertex_capacity: Slot,
    pub edges: Vec<Slot>,
    pub indices: Vec<Slot>, //Todo, make it contain EHandles which are not compatible with VHandles
}


impl Header {
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr_mut (edges: &mut Vec<Slot>, index: usize) -> (*mut Self, *mut VHandle) {
        let edges_ptr = edges.as_mut_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut VHandle;
            return (header_ptr, data_ptr);
        }
    }
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr (edges: &Vec<Slot>, index: usize) -> (*const Self, *const VHandle) {
        let edges_ptr = edges.as_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const VHandle;
            return (header_ptr, data_ptr);
        }
    }
    pub fn parse_mut (edges: &mut Vec<Slot>, index: usize) -> (&mut Self, &mut [VHandle]) {
        profile_method!(parse_mut);
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
        profile_method!(parse);
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


impl EdgeStorage {

    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_large() -> Self {
        return EdgeStorage {
            global_visited_flag: 1,
            vertex_capacity: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: Slot) -> Self {
        return EdgeStorage {
            global_visited_flag: 1,
            vertex_capacity: capacity,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. No reserve.
    pub fn new() -> Self {
        return EdgeStorage {
            global_visited_flag: 1,
            vertex_capacity: 0,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn len_mut_ptr(&mut self, vertex: VHandle) -> *mut Slot {
        return &mut self.edges[ (self.indices[vertex as usize] + LEN_OFFSET) as usize];
    }

    #[allow(unused)]
    fn reserve_mut(&mut self, vertex: VHandle) -> &mut Slot {
        let edge_chunk_index = self.indices[vertex as usize];
        return &mut self.edges[ (edge_chunk_index + CAPACITY_OFFSET) as usize];
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len_mut(&mut self, vertex: VHandle) -> &mut Slot {
        let edge_chunk_index = self.indices[vertex as usize];
        return &mut self.edges[ (edge_chunk_index + LEN_OFFSET) as usize];
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn calculate_new_edges_size_abs(&self, size: Slot) -> Slot {
        let header_size = HEADER_SIZE;
        return (self.edges.len() as Slot + self.vertex_capacity + header_size + size) as Slot;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn capacity(&self) -> Slot {
        return self.edges.len() as Slot;
    }
}

impl Operate for EdgeStorage {
    fn add_edges(&mut self, src: VHandle, targets: &[PackedEdge]) {
        let len = self.len(src) as usize;
        let new_size = len + targets.len();

        // TODO return as Result instead of panic!
        if new_size > self.edge_block_capacity(src) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.edges_mut(src);
        data[len..new_size].copy_from_slice(targets);
        *self.len_mut(src) = new_size as Slot;
    }

    fn extend_edge_storage(&mut self, size: Slot) -> Slot {
        let offset = self.edges.len() as Slot;
        let val = self.calculate_new_edges_size_abs(size);
        self.edges.resize_with(val as usize, Default::default);
        self.edges[ (offset + CAPACITY_OFFSET) as usize] = self.vertex_capacity + size;
        self.indices.push(offset);
        return (self.indices.len() - 1) as Slot;
    }

    fn disconnect(&mut self, src: VHandle, target: VHandle) {
        let data = self.edges_mut_ptr(src);
        let len = self.len_mut_ptr(src);
        unsafe {
            let mut iter = data;
            let end = iter.add(*len as usize);
            while iter != end{
                if vh(*iter) == target {
                    *iter = *end.offset(-1 ); // Swap the last element for the empty one
                    *len -= 1;
                    break;
                }
                iter = iter.offset(1);
            }
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn connect(&mut self, src: VHandle, target: VHandle) {
        self.add_edges(src, &[pack(target, 0)]);
    }
}

impl WeightedOperate for EdgeStorage{
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight) {
        self.add_edges(from, &[pack(to, weight)]);
    }
}

impl Visit for EdgeStorage {
    fn global_visited_flag(&self) -> Slot {
        return self.global_visited_flag;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn inc_global_visited_flag(&mut self) {
        self.global_visited_flag += 1;
    }

    fn reset_global_visited_flag(&mut self) {
        self.global_visited_flag = 0;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn visited_flag(&self, vertex: VHandle) -> Slot {
        profile_method!(visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return self.edges[ (edge_chunk_index as Slot + FLAG_OFFSET) as usize];
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn inc_visited_flag(&mut self, vertex: VHandle) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[ (edge_chunk_index as Slot + FLAG_OFFSET) as usize] += 1;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_visited_flag(&mut self, vertex: VHandle, val: Slot) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize];
        self.edges[ (edge_chunk_index + FLAG_OFFSET) as usize] = val;
    }
}

impl Store for EdgeStorage {
    fn edges_offset(&self, vertex: VHandle, offset: Slot) -> &[PackedEdge] {
        profile_method!(edges_from_offset);
        let edge_chunk_index = self.indices[vertex as usize];
        let len = self.edges[ (edge_chunk_index + LEN_OFFSET) as usize];
        let data = &self.edges[ (offset + edge_chunk_index + HEADER_SIZE) as usize.. (edge_chunk_index + HEADER_SIZE + len) as usize];
        return data;
    }
    fn edges_ptr_offset(&self, vertex: VHandle, offset: Slot) -> *const PackedEdge {
        profile_method!(edges_ptr_offset);
        let edge_chunk_index = self.indices[vertex as usize];
        return unsafe {self.edges.as_ptr().add((offset + edge_chunk_index + HEADER_SIZE) as usize)}
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges(&self, vertex: VHandle) -> &[PackedEdge] {
        return self.edges_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_ptr(&self, vertex: VHandle) -> *const PackedEdge {
        return self.edges_ptr_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len(&self, handle: VHandle) -> Slot {
        return self.edges[ (self.indices[handle as usize] + LEN_OFFSET) as usize];
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edge_block_capacity(&self, handle: VHandle) -> Slot {
        return self.edges[ (self.indices[handle as usize] + CAPACITY_OFFSET) as usize];
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get(&self, vertex: VHandle, offset: Slot) -> PackedEdge {
        let index = self.indices[vertex as usize];
        return self.edges[ ( index + HEADER_SIZE + offset) as usize];
    }

    fn edges_mut_offset(&mut self, vertex: VHandle, offset: Slot) -> &mut [PackedEdge] {
        profile_method!(edges_mut_from_offset);
        let edge_chunk_index = self.indices[vertex as usize];
        let reserve = self.edges[ (edge_chunk_index + CAPACITY_OFFSET) as usize];
        let data = &mut self.edges[ (offset + edge_chunk_index + HEADER_SIZE) as usize..(edge_chunk_index + HEADER_SIZE + reserve) as usize];
        return data;
    }
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: Slot) -> *mut PackedEdge {
        profile_method!(edges_mut_ptr_offset);
        let edge_chunk_index = self.indices[vertex as usize];
        let data = &mut self.edges[ (offset + edge_chunk_index + HEADER_SIZE) as usize];
        return data;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut PackedEdge {
        return self.edges_mut_ptr_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [PackedEdge] {
        return self.edges_mut_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set(&mut self, src: VHandle, val: PackedEdge, offset: Slot) {
        let index = self.indices[src as usize];
        self.edges[ (index + offset + HEADER_SIZE) as usize] = val;
    }
}
impl Clone for EdgeStorage {
    fn clone(&self) -> Self {
        return EdgeStorage {
            global_visited_flag: self.global_visited_flag,
            vertex_capacity: self.vertex_capacity,
            edges: self.edges.clone(),
            indices: self.indices.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.global_visited_flag = source.global_visited_flag;
        self.vertex_capacity = source.vertex_capacity;
        self.edges.clone_from(&source.edges);
        self.indices.clone_from(&source.indices);
    }
}

impl Manipulate for EdgeStorage {}

impl WeightedManipulate for EdgeStorage {}