use std::cmp::min;
use std::mem::{size_of};
use std::ops::{Index, IndexMut};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::thread::available_parallelism;
use crate::traits;
use crate::utils::{split_to_parts_mut};
use crate::views::tree::TreeView;

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

#[repr(C)]
pub struct Header{
    size: MSize,
    visited_flag: MSize,
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
    edge_capacity: usize,
    edges: Vec<MSize>,
    indices: Vec<MSize>,
}

impl Header {
    pub fn parse_mut (edge_chunk: &mut [MSize]) -> (&mut Self, &mut [MSize]) {
        let header_ptr = edge_chunk.as_ptr() as *mut Header;
        unsafe{
            let data_ptr = edge_chunk.as_mut_ptr().offset(header_size_to_elements() as isize);
            let data_slice = from_raw_parts_mut(data_ptr, (*header_ptr).size as usize);
            return (header_ptr.as_mut().unwrap(), data_slice);
        }
    }

    //TODO Add safety checks
    pub fn parse (edge_chunk: & [MSize]) -> (&Self, &[MSize]) {
        let header_ptr = edge_chunk.as_ptr() as *const Header;
        unsafe{
            let data_ptr = edge_chunk.as_ptr().offset(header_size_to_elements() as isize);
            let data_slice = from_raw_parts(data_ptr, (*header_ptr).size as usize);
            return (header_ptr.as_ref().unwrap(), data_slice);
        }
    }
}

#[cfg_attr(release, inline(always))]
pub const fn header_size_to_elements() -> usize {
    size_of::<Header>() / size_of::<MSize>()
}


impl<T> Graph<T>{

    pub fn tree_view(&mut self) -> TreeView<T> {
        return TreeView::new(&mut self.edges, &mut self.vertices);
    }
    pub fn new() -> Self {
        return Graph{
            edges: EdgeData::new(),
            vertices: Vertices::new(),

        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Graph{
            edges: EdgeData::with_capacity(capacity),
            vertices: Vertices::new(),
        };
    }
    pub fn create_and_connect(&mut self, src_vertex: MSize, val: T) -> MSize {
        let new_vertex = self.create(val);
        self.edges.connect(src_vertex, new_vertex);
        return new_vertex;
    }
    pub fn create(&mut self, val: T) -> MSize {
        self.vertices.data.push(val);
        return self.edges.create_vertex();
    }

    pub fn bfs(&mut self, start: MSize) -> Vec<MSize> {
        let mut nodes: Vec<MSize> = Vec::new();
        nodes.push(start as MSize);
        let mut i = 0;
        while i < nodes.len() {
            let val = nodes[i];
            self.edges.inc_visited_flag(val);
            //This has to be always valid
            let edges = self.edges.edge_data(val).ok().unwrap();
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

    pub fn bfs_transform<F>(&mut self, start: MSize, mut transform: F)
    where F: FnMut(&mut Self, MSize){
        let mut nodes: Vec<MSize> = Vec::new();
        nodes.push(start as MSize);
        let mut i = 0;
        while i < nodes.len() {
            let val = nodes[i];
            transform(self, val);
            self.edges.inc_visited_flag(val);
            //This has to be always valid
            let edges = self.edges.edge_data(val).ok().unwrap();
            for next in edges {
                if self.edges.visited_flag(*next) == self.edges.visited_val {
                    continue;
                }
                nodes.push(*next);
            }
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

    pub fn new() -> Self {
        return EdgeData{
            visited_val: 1,
            edge_capacity: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        return EdgeData{
            visited_val: 1,
            edge_capacity: capacity,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn add_edges(&mut self, vertex: MSize, new_edges: &[MSize]) {
        let data_start_index = self.indices[vertex as usize] as usize;
        let edges_count = self.edges[data_start_index] as usize;
        let new_size = edges_count + new_edges.len();

        if new_size > self.edge_capacity {
            panic!("Edge array full!");
        }

        if new_size > self.edges.len() {
            panic!("Edge size is greater than the allocated size");
        }

        let new_data_start = data_start_index + edges_count + header_size_to_elements();
        let new_data_end = new_data_start + new_edges.len();


        self.edges[new_data_start..new_data_end].copy_from_slice(new_edges);
        self.edges[data_start_index] = new_size as MSize;
    }

    #[cfg_attr(release, inline(always))]
    fn calculate_new_edges_size(&self) -> usize {
        return self.edges.len() + self.edge_capacity + (header_size_to_elements());
    }
    pub fn create_vertex(&mut self) -> MSize{
        let old_size = self.edges.len() as MSize;

        self.edges.resize_with(self.calculate_new_edges_size() as usize, Default::default);
        self.indices.push(old_size);
        return (self.indices.len() - 1) as MSize;
    }

    //TODO Add checks for unsafe

    pub fn disconnect(&mut self, src: MSize, vertex: MSize) {
        let edges_index = self.indices[src as usize] as usize;

        unsafe {
            let data_start = &mut self.edges[edges_index] as *mut MSize;
            let size = data_start;
            let mut iter = data_start.offset(header_size_to_elements() as isize);
            let end = iter.offset(*size as isize);
            while iter != end{
                if *iter == vertex{
                    *iter = *end.offset(-1); // Swap the last element for the empty one
                    *size -= 1;
                    break;
                }
                iter = iter.offset(1);
            }
        }
    }


    // This is the safe version, but it sucks because it involves direct indexing
    // Performed measurements, the safe version is taking ~362.1235ms on 20 000 elements while the unsafe version is taking ~77.066ms on Debug
    // On --release for Ryzen 7900x, the safe version and unsafe version around ~18ms
    // On --release for Core(TM) i7-1165G7 @ 2.80GHz the safe version is ~65.4733ms unsafe is 24.0857ms
    // That is ~2x improvement in performance in unsafe. It also scales better with lower-end hardware.
    // Fuck safe!
    pub fn disconnect_safe(&mut self, src: MSize, vertex: MSize) {
        let edges_index = self.indices[src as usize];
        let edge_data = &mut self.edges[edges_index as usize..];
        let header_size = header_size_to_elements();

        if edge_data.len() <= header_size {
            return;
        }
        let size = edge_data[0] as usize;
        let data_len = size + header_size;
        let data_range = &mut edge_data[header_size..data_len];

        for i in 0..size {
            if data_range[i] == vertex {
                data_range[i] = *data_range.last().unwrap();
                edge_data[0] -= 1; // Decrease the size
                break;
            }
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn set(&mut self, src: MSize, vertex: MSize, position: usize){
        let edges = self.edges_mut(src);
        if edges.is_err() {
            panic!("Vertex not found!");
        }
        let edges = edges.ok().unwrap();
        edges[position] = vertex;

    }

    pub fn edges_mut(&mut self, vertex: MSize) -> Result< &mut [MSize], Error>{
        let uvertex = vertex as usize;
        let edge = self.indices[uvertex] as usize;
        let size = self.edges[edge] as usize;

        if uvertex > self.edges.len() {
            return Err(Error::NoHandle);
        }

        return Ok(&mut self.edges[edge + header_size_to_elements()..edge + size + header_size_to_elements() ]);
    }


    #[cfg_attr(release, inline(always))]
    pub fn len(&self, vertex: MSize) -> MSize {
        return self.edges[self.indices[vertex as usize] as usize];
    }

    #[cfg_attr(release, inline(always))]
    pub fn connect(&mut self, from: MSize, to: MSize) {
        self.add_edges(from, &[to]);
    }
    #[cfg_attr(release, inline(always))]
    pub fn capacity(&self) -> usize {
        return self.edges.len();
    }

    //TODO Change to tuple of header + edges
    pub fn edge_data(&self, vertex: MSize) -> Result< &[MSize], Error> {
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }

        let edge = self.indices[uvertex] as usize;
        let size = self.edges[edge] as usize;
        return Ok(&self.edges[edge + header_size_to_elements()..edge + size + header_size_to_elements() ]);
    }

    pub fn edge_chunk(&self, vertex: MSize) -> Result< (&Header, &[MSize]) , Error> {
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        let edge_chunk_index = self.indices[uvertex] as usize;
        return Ok(Header::parse(self.edges.split_at(edge_chunk_index).1));
    }

    fn edge_chunk_mut(&mut self, vertex: MSize) -> Result< (&mut Header, &mut [MSize]) , Error> {
        let uvertex = vertex as usize;
        if uvertex > self.indices.len() {
            return Err(Error::NoHandle);
        }
        let edge_chunk_index = self.indices[uvertex] as usize;
        return Ok(Header::parse_mut(self.edges.split_at_mut(edge_chunk_index).1));
    }

    #[cfg_attr(release, inline(always))]
    fn inc_visited_flag(&mut self, vertex: MSize) {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index+1] += 1; // The flag is at offset 1
    }
    #[cfg_attr(release, inline(always))]
    fn set_visited_flag(&mut self, vertex: MSize, val: MSize) {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        self.edges[edge_chunk_index+1] = val; // The flag is at offset 1
    }
    #[cfg_attr(release, inline(always))]
    fn visited_flag(&self, vertex: MSize) -> MSize {
        let edge_chunk_index = self.indices[vertex as usize] as usize;
        return self.edges[edge_chunk_index+1]; // The flag is at offset 1
    }


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

