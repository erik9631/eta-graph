use std::cmp::min;
use std::mem::{size_of, transmute};
use std::ops::{Index, IndexMut};
use std::ptr::slice_from_raw_parts_mut;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::thread::available_parallelism;
use firestorm::{profile_fn, profile_method, profile_section};
use crate::traits;
use crate::utils::{split_to_parts_mut};
use crate::views::tree::TreeView;

#[derive(Debug)]
pub enum Error {
    NoHandle,
}

pub enum TraverseResult {
    Continue,
    End,
}

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

pub struct Vertices<T> {
    data: Vec<T>,
}

pub struct Graph<T> {
    pub vertices: Vertices<T>,
    pub edges: EdgeData,
}


pub struct EdgeData{
    visited_val: MSize, // Val used to mark whether the vertex has been visited
    reserve: usize,
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
impl<T> Graph<T>{

    pub fn tree_view(&mut self) -> TreeView<T> {
        return TreeView::new(&mut self.edges, &mut self.vertices);
    }

    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_large() -> Self {
        return Graph{
            edges: EdgeData::new_dyn(),
            vertices: Vertices::new(),

        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(reserve: usize) -> Self {
        return Graph{
            edges: EdgeData::with_reserve(reserve),
            vertices: Vertices::new(),
        };
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. Small reserve count of 5
    pub fn new() -> Self {
        return Graph{
            edges: EdgeData::new(),
            vertices: Vertices::new(),
        };
    }

    pub fn create_and_connect(&mut self, src_vertex: MSize, val: T, edge_count: usize) -> MSize {
        let new_vertex = self.create(val, edge_count);
        self.edges.connect(src_vertex, new_vertex);
        return new_vertex;
    }
    pub fn create_and_connect_leaf(&mut self, src_vertex: MSize, val: T) -> MSize {
        return self.create_and_connect(src_vertex, val, 0);
    }

    pub fn create(&mut self, val: T, edge_count: usize) -> MSize {
        self.vertices.push(val);
        let new_vertex = (self.vertices.len() - 1)  as MSize;
        self.edges.create_vertex(edge_count);
        return new_vertex;
    }
    #[cfg_attr(release, inline(always))]
    pub fn create_leaf(&mut self, val: T) -> MSize {
        return self.create(val, 0)
    }

    pub fn bfs_vec(&mut self, start: MSize) -> Vec<MSize> {
        let mut nodes: Vec<MSize> = Vec::new();
        nodes.push(start as MSize);
        let mut i = 0;
        while i < nodes.len() {
            let val = nodes[i];
            self.edges.inc_visited_flag(val);
            //This has to be always valid
            let edges = self.edges.edges(val);
            for next in edges {
                if self.edges.visited_flag(*next) == self.edges.visited_val {
                    continue;
                }
                nodes.push(*next);
            }
            i +=1;
        }
        self.edges.visited_val = 0; // Reset the visited flag as we traversed the whole graph
        return nodes;
    }

    pub fn bfs<F>(&mut self, start: MSize, mut transform: F)
    where F: FnMut(&mut Self, MSize){
        profile_method!(bfs);
        profile_section!(before_loop);
        let mut nodes: Vec<MSize> = Vec::with_capacity(self.vertices.len());
        nodes.push(start as MSize);
        let mut i = 0;
        drop(before_loop);
        profile_section!(loop_start);
        while i < nodes.len() {
            profile_section!(loop_before_inner);
            let val = unsafe {*nodes.get_unchecked(i)};
            transform(self, val);
            self.edges.inc_visited_flag(val);

            //This has to be always valid
            let edges = self.edges.edges(val);
            drop(loop_before_inner);
            profile_section!(loop_inner);
            for next in edges {
                if self.edges.visited_flag(*next) == self.edges.visited_val {
                    continue;
                }
                nodes.push(*next);
            }
            drop(loop_inner);
            profile_section!(increment_i);
            i +=1;
        }
        self.edges.visited_val = 0; // Reset the visited flag as we traversed the whole graph
    }
}


impl <T: Send> traits::Transform<T> for Vertices<T> {
    fn transform(&mut self, transform_fn: fn(&mut [T])) {
        transform_fn(self.data.as_mut_slice());
    }
    fn async_transform(&mut self, transform_fn: fn(&mut [T])) {
        let max_parallelism = available_parallelism().ok().unwrap().get();
        let parallelism_count = min(max_parallelism, self.data.len());
        let parts = split_to_parts_mut(&mut self.data, parallelism_count);

        std::thread::scope(|scope| {
            for part in parts {
                scope.spawn(|| {
                    transform_fn(part);
                });
            }
        });


    }

}
impl <T> Vertices<T>{
    pub fn new() -> Self {
        return Vertices{
            data: Vec::new(),
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }
    pub fn len(&self) -> usize {
        return self.data.len();
    }
}

impl <T> Index<MSize> for Vertices<T>{
    type Output = T;
    fn index(&self, index: MSize) -> &Self::Output {
        return &self.data[index as usize];
    }
}

impl <T> IndexMut<MSize> for Vertices<T>{
    fn index_mut(&mut self, index: MSize) -> &mut Self::Output {
        return &mut self.data[index as usize];
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
        let (header, data) = Header::parse_mut(&mut self.edges, self.indices[vertex as usize] as usize);
        let new_size = header.len as usize + new_edges.len();

        // TODO return as Result instead of panic!
        if new_size > header.reserve as usize {
            panic!("Edge size is greater than the allocated size");
        }
        let new_data_end = header.len as usize + new_edges.len();

        data[header.len as usize..new_data_end].copy_from_slice(new_edges);
        header.len = new_size as MSize;
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
    pub fn len(&self, vertex: MSize) -> MSize {
        let (header, _) = Header::parse(&self.edges, self.indices[vertex as usize] as usize);
        return header.len;
    }

    #[cfg_attr(release, inline(always))]
    pub fn connect(&mut self, from: MSize, to: MSize) {
        self.add_edges(from, &[to]);
    }
    #[cfg_attr(release, inline(always))]
    pub fn capacity(&self) -> usize {
        return self.edges.len();
    }

    pub fn reserve(&mut self, vertex: MSize) -> MSize {
        unsafe {
            let header = &self.edges[self.indices[vertex as usize] as usize] as *const MSize;
            let header_ptr: *const Header = transmute(header);
            let reserve = (*header_ptr).reserve;
            return reserve;
        }
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
    pub fn edges(&self, vertex: MSize) -> &[MSize] {
        profile_method!(edges_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let len = self.edges[edge_chunk_index + LEN_OFFSET] as usize;
        let data = &self.edges[edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + len];
        return data;
    }

    pub fn edges_header_mut(&mut self, vertex: MSize) -> Result< &mut [MSize], Error>{
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        let (_, data) = Header::parse_mut(&mut self.edges, self.indices[uvertex] as usize);
        return Ok(data);
    }

    pub fn edges_mut(&mut self, vertex: MSize) -> &mut [MSize] {
        profile_method!(edges_fast);
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        let len = self.edges[edge_chunk_index + LEN_OFFSET] as usize;
        let data = &mut self.edges[edge_chunk_index + DATA_START_OFFSET..edge_chunk_index + DATA_START_OFFSET + len];
        return data;
    }
}


#[cfg_attr(release, inline(always))]
pub fn header_size_in_msize_units() -> usize {
    let raw_size = size_of::<Header>();
    ((raw_size + MSIZE_ALIGN_MASK) & !MSIZE_ALIGN_MASK) / size_of::<MSize>()
}


// pub fn dfs<T>(root: &Tree<T>, traverse: fn(node: &Tree<T>)){
//     let mut stack: Vec<(&Tree<T>, Iter<*mut Tree<T>>)> = Vec::new();
//     stack.push( (root, root.children.iter()));
//
//     while !stack.is_empty() {
//         let current_node = stack.last_mut().unwrap();
//
//         let child_node = current_node.1.next();
//         let parent_node = current_node.0;
//         match child_node {
//             None => {
//                 stack.pop();
//                 traverse(parent_node);
//             },
//             Some(child_node) => {
//                 stack.push( (child_node, child_node.children.iter()) );
//             }
//         }
//     }
// }

