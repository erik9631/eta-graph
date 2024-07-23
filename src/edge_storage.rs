use std::mem::size_of;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use firestorm::{profile_method, profile_section};
use crate::graph::{Error};
use crate::handles::{MSIZE_ALIGN_MASK};
use crate::handles::types::{VertId, VHandle, Weight};
use crate::traits::{EdgeOperator, EdgeStore, EdgeStoreMut, TraverseMarker, WeightedEdgeOperator};

const FLAG_OFFSET: usize = 0;
const LEN_OFFSET: usize = 1;
const CAPACITY_OFFSET: usize = 2;
const DATA_START_OFFSET: usize = 3;
#[repr(C)]
pub struct Header{
    pub visited_flag: VHandle,
    pub len: VHandle,
    pub capacity: VHandle,
}

pub struct EdgeStorage {
    pub (in crate) global_visited_flag: VHandle, // Val used to mark whether the vertex has been visited
    pub(in crate) vertex_capacity: usize,
    pub edges: Vec<VHandle>,
    pub indices: Vec<VHandle>, //Todo, make it contain EHandles which are not compatible with VHandles
}


impl Header {
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr_mut (edges: &mut Vec<VHandle>, index: usize) -> (*mut Self, *mut VHandle) {
        let edges_ptr = edges.as_mut_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut VHandle;
            return (header_ptr, data_ptr);
        }
    }
    #[allow(unused)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn parse_ptr (edges: &Vec<VHandle>, index: usize) -> (*const Self, *const VHandle) {
        let edges_ptr = edges.as_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const VHandle;
            return (header_ptr, data_ptr);
        }
    }
    pub fn parse_mut (edges: &mut Vec<VHandle>, index: usize) -> (&mut Self, &mut [VHandle]) {
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
    pub fn parse (edges: &Vec<VHandle>, index: usize) -> (&Self, &[VHandle]) {
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
    pub const NONE: VHandle = VHandle::MAX;

    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_dyn() -> Self {
        return EdgeStorage {
            global_visited_flag: 1,
            vertex_capacity: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: usize) -> Self {
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

    fn len_mut_ptr(&mut self, vertex: VHandle) -> *mut VHandle {
        return &mut self.edges[self.indices[vertex as usize] as usize + LEN_OFFSET];
    }

    #[allow(unused)]
    fn reserve_mut(&mut self, vertex: VHandle) -> &mut VHandle {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return &mut self.edges[edge_chunk_index + CAPACITY_OFFSET];
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len_mut(&mut self, vertex: VHandle) -> &mut VHandle {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return &mut self.edges[edge_chunk_index + LEN_OFFSET];
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn calculate_new_edges_size_abs(&self, size: usize) -> usize {
        let header_size = header_size_in_msize_units();
        return self.edges.len() + self.vertex_capacity + header_size + size;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn capacity(&self) -> usize {
        return self.edges.len();
    }
    pub fn edges_header(&self, vertex: VHandle) -> Result< &[VHandle], Error> {
        profile_method!(edges);
        profile_section!(before_parse);
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        drop(before_parse);
        profile_section!( nd_return);
        let (_, data) = Header::parse(&self.edges, self.indices[uvertex] as usize);
        return Ok(data);
    }

    pub fn edges_header_mut(&mut self, vertex: VHandle) -> Result< &mut [VHandle], Error>{
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        let (_, data) = Header::parse_mut(&mut self.edges, self.indices[uvertex] as usize);
        return Ok(data);
    }
}

impl EdgeOperator for EdgeStorage {
    fn add_edges(&mut self, handle: VHandle, handle_list: &[VHandle]) {
        let len = self.len(handle) as usize;
        let new_size = len + handle_list.len();

        // TODO return as Result instead of panic!
        if new_size > self.edge_block_capacity(handle) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.edges_mut(handle);
        data[len..new_size].copy_from_slice(handle_list);
        *self.len_mut(handle) = new_size as VHandle;
    }

    fn extend_edge_storage(&mut self, size: usize) -> VHandle{
        let offset = self.edges.len() as VHandle;
        let val = self.calculate_new_edges_size_abs(size);
        self.edges.resize_with(val, Default::default);
        self.edges[offset as usize + CAPACITY_OFFSET] = self.vertex_capacity as VHandle + size as VHandle;
        self.indices.push(offset);
        return (self.indices.len() - 1) as VHandle;
    }

    fn disconnect(&mut self, src: VHandle, target: VHandle) {
        let data = self.edges_mut_ptr(src);
        let len = self.len_mut_ptr(src);
        unsafe {
            let mut iter = data;
            let end = iter.add(*len as usize);
            while iter != end{
                if *iter == target {
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
        self.add_edges(src, &[target]);
    }
}

impl WeightedEdgeOperator for EdgeStorage{
    fn connect_weighted(&mut self, from: VHandle, to: VHandle, weight: Weight) {
        let combined: VHandle = to | ( (weight as VHandle) << size_of::<VertId>());
        self.add_edges(from, &[combined]);
    }
}

impl TraverseMarker for EdgeStorage {
    fn global_visited_flag(&self) -> VHandle {
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
    fn visited_flag(&self, vertex: VHandle) -> VHandle {
        profile_method!(visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return self.edges[edge_chunk_index + FLAG_OFFSET];
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn inc_visited_flag(&mut self, vertex: VHandle) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index + FLAG_OFFSET] += 1;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_visited_flag(&mut self, vertex: VHandle, val: VHandle) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index + FLAG_OFFSET] = val;
    }
}

impl EdgeStore for EdgeStorage {
    fn edges_offset(&self, vertex: VHandle, offset: usize) -> &[VHandle]{
        profile_method!(edges_from_offset);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let len = self.edges[edge_chunk_index + LEN_OFFSET] as usize;
        let data = &self.edges[offset + edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + len];
        return data;
    }
    fn edges_ptr_offset(&self, vertex: VHandle, offset: usize) -> *const VHandle {
        profile_method!(edges_ptr_offset);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let data = &self.edges[offset + edge_chunk_index + DATA_START_OFFSET];
        return data;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges(&self, vertex: VHandle) -> &[VHandle] {
        return self.edges_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_ptr(&self, vertex: VHandle) -> *const VHandle {
        return self.edges_ptr_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len(&self, handle: VHandle) -> VHandle {
        return self.edges[self.indices[handle as usize] as usize + LEN_OFFSET];
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edge_block_capacity(&self, handle: VHandle) -> usize {
        return self.edges[self.indices[handle as usize] as usize + CAPACITY_OFFSET] as usize;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get(&self, handle: VHandle, offset: usize) -> VHandle {
        let index = self.indices[handle as usize] as usize;
        return self.edges[index + DATA_START_OFFSET + offset];
    }
}
impl EdgeStoreMut for EdgeStorage {
    fn edges_mut_offset(&mut self, vertex: VHandle, offset: usize) -> &mut [VHandle]{
        profile_method!(edges_mut_from_offset);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let reserve = self.edges[edge_chunk_index + CAPACITY_OFFSET] as usize;
        let data = &mut self.edges[offset + edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + reserve];
        return data;
    }
    fn edges_mut_ptr_offset(&mut self, vertex: VHandle, offset: usize) -> *mut VHandle {
        profile_method!(edges_mut_ptr_offset);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let data = &mut self.edges[offset + edge_chunk_index + DATA_START_OFFSET];
        return data;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut_ptr(&mut self, vertex: VHandle) -> *mut VHandle {
        return self.edges_mut_ptr_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn edges_mut(&mut self, vertex: VHandle) -> &mut [VHandle] {
        return self.edges_mut_offset(vertex, 0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set(&mut self, src_handle: VHandle, val: VHandle, offset: usize){
        let index = self.indices[src_handle as usize] as usize;
        self.edges[index + offset + DATA_START_OFFSET] = val;
    }

}


#[cfg_attr(not(debug_assertions), inline(always))]
pub fn header_size_in_msize_units() -> usize {
    let raw_size = size_of::<Header>();
    ((raw_size + MSIZE_ALIGN_MASK) & !MSIZE_ALIGN_MASK) / size_of::<VHandle>()
}