use std::mem::size_of;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use firestorm::{profile_method, profile_section};
use crate::graph::{Error};

#[cfg(msize_type = "u8")]
pub type MSize = u8;

#[cfg(msize_type = "u16")]
pub type MSize = u16;

#[cfg(msize_type = "u32")]
pub type MSize = u32;

#[cfg(msize_type = "usize")]
pub type MSize = usize;

const MSIZE_ALIGN_MASK: usize = size_of::<MSize>() - 1;

const FLAG_OFFSET: usize = 0;
const LEN_OFFSET: usize = 1;
const RESERVE_OFFSET: usize = 2;
const DATA_START_OFFSET: usize = 3;
#[repr(C)]
pub struct Header{
    pub visited_flag: MSize,
    pub len: MSize,
    pub reserve: MSize,
}

pub struct EdgeData{
    pub (in crate) visited_val: MSize, // Val used to mark whether the vertex has been visited
    pub(in crate) reserve: usize,
    pub edges: Vec<MSize>,
    pub indices: Vec<MSize>,
}


impl Header {
    #[cfg_attr(release, inline(always))]
    pub fn parse_ptr_mut (edges: &mut Vec<MSize>, index: usize) -> (*mut Self, *mut MSize) {
        let edges_ptr = edges.as_mut_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut MSize;
            return (header_ptr, data_ptr);
        }
    }
    #[cfg_attr(release, inline(always))]

    pub fn parse_ptr (edges: &Vec<MSize>, index: usize) -> (*const Self, *const MSize) {
        let edges_ptr = edges.as_ptr();
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const MSize;
            return (header_ptr, data_ptr);
        }
    }
    pub fn parse_mut (edges: &mut Vec<MSize>, index: usize) -> (&mut Self, &mut [MSize]) {
        profile_method!(parse_mut);
        let edges_ptr = edges.as_mut_ptr();

        // Return as Result instead of panic
        if index >= edges.len() {
            panic!("Index out of bounds");
        }

        unsafe{
            let header_ptr = edges_ptr.add(index) as *mut Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *mut MSize;
            let data = from_raw_parts_mut(data_ptr, (*header_ptr).reserve as usize);
            return (header_ptr.as_mut().unwrap(), data);
        }
    }
    pub fn parse (edges: &Vec<MSize>, index: usize) -> (&Self, &[MSize]) {
        profile_method!(parse);
        // Return as Result instead of panic
        let edges_ptr = edges.as_ptr();

        if index >= edges.len() {
            panic!("Index out of bounds");
        }
        unsafe{
            let header_ptr = edges_ptr.add(index) as *const Header;
            let data_ptr = edges_ptr.byte_add(size_of::<Header>()).add(index) as *const MSize;
            let data = from_raw_parts(data_ptr, (*header_ptr).len as usize);
            return (header_ptr.as_ref().unwrap(), data);
        }
    }
}


impl EdgeData {
    pub const NONE: MSize = MSize::MAX;
    const MSIZE_ALIGN_MASK: usize = size_of::<MSize>() - 1;

    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_dyn() -> Self {
        return EdgeData{
            visited_val: 1,
            reserve: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(capacity: usize) -> Self {
        return EdgeData{
            visited_val: 1,
            reserve: capacity,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. No reserve.
    pub fn new() -> Self {
        return EdgeData{
            visited_val: 1,
            reserve: 0,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add_edges(&mut self, vertex: MSize, new_edges: &[MSize]) {
        let len = self.len(vertex) as usize;
        let new_size = len + new_edges.len();

        // TODO return as Result instead of panic!
        if new_size > self.reserve(vertex) as usize {
            panic!("Edge size is greater than the allocated size");
        }

        let data = self.edges_mut(vertex);
        data[len..new_size].copy_from_slice(new_edges);
        *self.len_mut(vertex) = new_size as MSize;
    }

    #[cfg_attr(release, inline(always))]
    fn calculate_new_edges_size_abs(&self, size: usize) -> usize {
        let header_size = header_size_in_msize_units();
        return self.edges.len() + self.reserve + header_size + size;
    }
    pub fn create_vertex(&mut self, size: usize) -> MSize{
        let offset = self.edges.len() as MSize;
        let val = self.calculate_new_edges_size_abs(size);
        self.edges.resize_with(val, Default::default);
        unsafe{
            let header_ptr = self.edges.as_mut_ptr().add(offset as usize) as *mut Header;
            (*header_ptr).reserve = self.reserve as MSize + size as MSize;

        }
        self.indices.push(offset);
        return (self.indices.len() - 1) as MSize;
    }

    //TODO Add checks for unsafe
    pub fn disconnect(&mut self, src: MSize, vertex: MSize) {
        let edges_index = self.indices[src as usize] as usize;
        let (header, data) = Header::parse_ptr_mut(&mut self.edges, edges_index);

        unsafe {
            let mut iter = data;
            let end = iter.add((*header).len as usize);
            while iter != end{
                if *iter == vertex{
                    *iter = *end.offset(-1 ); // Swap the last element for the empty one
                    (*header).len -= 1;
                    break;
                }
                iter = iter.offset(1);
            }
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn set(&mut self, src: MSize, val: MSize, edge: usize){
        let edges = self.edges_mut(src);
        edges[edge] = val;
    }

    #[cfg_attr(release, inline(always))]
    pub fn get(&self, vertex: MSize, edge: usize) -> MSize{
        return self.edges[self.indices[vertex as usize] as usize + edge + header_size_in_msize_units()];
    }
    #[cfg_attr(release, inline(always))]
    fn len_mut(&mut self, vertex: MSize) -> &mut MSize {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return &mut self.edges[edge_chunk_index + LEN_OFFSET];
    }

    fn reserve_mut(&mut self, vertex: MSize) -> &mut MSize {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return &mut self.edges[edge_chunk_index + RESERVE_OFFSET];
    }


    #[cfg_attr(release, inline(always))]
    pub fn len(&self, vertex: MSize) -> MSize {
        return self.edges[self.indices[vertex as usize] as usize + LEN_OFFSET];
    }

    #[cfg_attr(release, inline(always))]
    pub fn connect(&mut self, from: MSize, to: MSize) {
        self.add_edges(from, &[to]);
    }
    #[cfg_attr(release, inline(always))]
    pub fn capacity(&self) -> usize {
        return self.edges.len();
    }
    #[cfg_attr(release, inline(always))]
    pub fn reserve(&self, vertex: MSize) -> MSize {
        return self.edges[self.indices[vertex as usize] as usize + RESERVE_OFFSET];
    }
    #[cfg_attr(release, inline(always))]
    pub fn visited_flag(&self, vertex: MSize) -> MSize {
        profile_method!(visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return self.edges[edge_chunk_index + FLAG_OFFSET];
    }

    #[cfg_attr(release, inline(always))]
    pub fn inc_visited_flag(&mut self, vertex: MSize) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index + FLAG_OFFSET] += 1;
    }

    #[cfg_attr(release, inline(always))]
    pub fn set_visited_flag(&mut self, vertex: MSize, val: MSize) {
        profile_method!(inc_visited_flag_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index + FLAG_OFFSET] = val;
    }
    pub fn edges_header(&self, vertex: MSize) -> Result< &[MSize], Error> {
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
    pub fn edges_header_mut(&mut self, vertex: MSize) -> Result< &mut [MSize], Error>{
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        let (_, data) = Header::parse_mut(&mut self.edges, self.indices[uvertex] as usize);
        return Ok(data);
    }

    pub fn edges(&self, vertex: MSize) -> &[MSize] {
        profile_method!(edges_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let len = self.edges[edge_chunk_index + LEN_OFFSET] as usize;
        let data = &self.edges[edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + len];
        return data;
    }


    pub fn edges_mut(&mut self, vertex: MSize) -> &mut [MSize] {
        profile_method!(edges_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let reserve = self.edges[edge_chunk_index + RESERVE_OFFSET] as usize;
        let data = &mut self.edges[edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + reserve];
        return data;
    }
}


#[cfg_attr(release, inline(always))]
pub fn header_size_in_msize_units() -> usize {
    let raw_size = size_of::<Header>();
    ((raw_size + MSIZE_ALIGN_MASK) & !MSIZE_ALIGN_MASK) / size_of::<MSize>()
}
