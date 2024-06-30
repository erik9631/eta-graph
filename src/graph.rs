use std::cmp::min;
use std::mem;
use std::mem::{size_of};
use std::ops::{Index, IndexMut};
use std::thread::available_parallelism;
use crate::traits;
use crate::utils::{extract_from_slice_mut, split_to_parts_mut};
use crate::views::tree::TreeView;

pub enum Error{
    NoHandle,
}


#[repr(C)]
struct Header{
    size: usize,
    visited_flag: usize,
}

pub struct Vertices<T> {
    data: Vec<T>,
}

pub struct Graph<T> {
    pub vertices: Vertices<T>,
    pub edges: EdgeData,
}


pub struct EdgeData{
    edge_capacity: usize,
    edge_data: Vec<usize>,
    indices: Vec<usize>,
}

impl Header {
    pub fn parse_mut (edge_slice: &mut [usize]) -> &mut Self {
        let header_ptr = edge_slice.as_mut_ptr() as *mut Header;
        return unsafe {mem::transmute(header_ptr)};
    }

    pub fn parse (edge_slice: & [usize]) -> & Self {
        let header_ptr = edge_slice.as_ptr() as *mut Header;
        return unsafe {mem::transmute(header_ptr)};
    }
}

#[cfg_attr(release, inline(always))]
pub const fn header_size_to_elements() -> usize {
    size_of::<Header>() / size_of::<usize>()
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
    pub fn create_and_connect(&mut self, src_vertex: usize, val: T) -> usize {
        let new_vertex = self.create(val);
        self.edges.connect(src_vertex, new_vertex);
        return new_vertex;
    }
    pub fn create(&mut self, val: T) -> usize {
        self.vertices.data.push(val);
        return self.edges.create_vertex();
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

impl <T> Index<usize> for Vertices<T>{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

impl <T> IndexMut<usize> for Vertices<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}
impl EdgeData {
    pub const NONE: usize = usize::MAX;

    pub fn new() -> Self {
        return EdgeData{
            edge_capacity: 50,
            edge_data: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        return EdgeData{
            edge_capacity: capacity,
            edge_data: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn add_edges(&mut self, vertex: usize, new_edges: &[usize]) {
        let data_start_index = self.indices[vertex];
        let edges_count = self.edge_data[data_start_index];
        let new_size = edges_count + new_edges.len();

        if new_size > self.edge_capacity {
            panic!("Edge array full!");
        }

        if new_size > self.edge_data.len() {
            panic!("Edge size is greater than the allocated size");
        }

        let new_data_start = data_start_index + edges_count + header_size_to_elements();
        let new_data_end = new_data_start + new_edges.len();


        self.edge_data[new_data_start..new_data_end].copy_from_slice(new_edges);
        self.edge_data[data_start_index] = new_size;
    }

    #[cfg_attr(release, inline(always))]
    fn calculate_new_edges_size(&self) -> usize {
        return self.edge_data.len() + self.edge_capacity + (header_size_to_elements());
    }
    pub fn create_vertex(&mut self) -> usize{
        let old_size = self.edge_data.len();

        self.edge_data.resize_with(self.calculate_new_edges_size(), Default::default);
        self.indices.push(old_size);
        return self.indices.len() - 1;
    }

    //TODO Add checks for unsafe

    pub fn disconnect(&mut self, src: usize, vertex: usize) {
        let edges_index = self.indices[src];

        unsafe {
            let data_start = &mut self.edge_data[edges_index] as *mut usize;
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

    #[cfg_attr(release, inline(always))]
    pub fn set(&mut self, src: usize, vertex: usize, position: usize){
        let edges = self.edges_mut(src);
        if edges.is_err() {
            panic!("Vertex not found!");
        }
        let edges = edges.ok().unwrap();
        edges[position] = vertex;

    }

    pub fn edges_mut(&mut self, vertex: usize) -> Result< &mut [usize], Error>{
        let edge = self.indices[vertex];
        let size = self.edge_data[edge];

        if vertex > self.edge_data.len() {
            return Err(Error::NoHandle);
        }

        return Ok(&mut self.edge_data[edge + header_size_to_elements()..edge + size + header_size_to_elements() ]);
    }


    #[cfg_attr(release, inline(always))]
    pub fn len(&self, vertex: usize) -> usize {
        return self.edge_data[self.indices[vertex]];
    }

    #[cfg_attr(release, inline(always))]
    pub fn connect(&mut self, from: usize, to: usize) {
        self.add_edges(from, &[to]);
    }
    #[cfg_attr(release, inline(always))]
    pub fn capacity(&self) -> usize {
        return self.edge_data.len();
    }
    pub fn edges(&self, vertex: usize) -> Result< &[usize], Error> {
        if vertex > self.indices.len() {
            return Err(Error::NoHandle);
        }

        let edge = self.indices[vertex];
        let size = self.edge_data[edge];
        return Ok(&self.edge_data[edge + header_size_to_elements()..edge + size + header_size_to_elements() ]);
    }

}

// pub fn bfs<T>(root: *mut Tree<T>, traverse: fn(node: &Tree<T>)){
//     let mut nodes: LinkedList<*mut Tree<T>> = LinkedList::new();
//     nodes.push_back(root);
//
//     while !nodes.is_empty() {
//         let node = nodes.front();
//         if node.is_none() {
//             return;
//         }
//         let node = node.unwrap();
//         traverse(node);
//
//         for child in node.get_children().iter() {
//             nodes.push_back(*child);
//         }
//     }
// }

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

