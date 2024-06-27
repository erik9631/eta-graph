use std::cmp::min;
use std::mem::{size_of};
use std::ops::{Index, IndexMut};
use std::thread::available_parallelism;
use crate::traits;
use crate::utils::{split_to_mut_parts};

pub enum Error{
    NoHandle,
}

type Header = usize;
#[cfg_attr(release, inline(always))]
fn header_element_size() -> usize {
    return size_of::<Header>() / size_of::<usize>();
}

pub struct Graph<T> {
    pub vertices: Vertices<T>,
    pub edges: EdgeData,
}
impl<T> Graph<T>{
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
        }
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


pub struct Vertices<T> {
    data: Vec<T>,
}

impl <T: Send> traits::Transform<T> for Vertices<T> {
    fn transform(&mut self, transform_fn: fn(&mut T)) {

        for i in 0..self.data.len() {
            transform_fn(&mut self.data[i]);
        }
    }
    fn async_transform(&mut self, transform_fn: fn(&mut T)) {
        let max_parallelism = available_parallelism().ok().unwrap().get();
        let parallelism_count = min(max_parallelism, self.data.len());
        let parts = split_to_mut_parts(&mut self.data, parallelism_count);

        for part in parts {
            std::thread::scope(|scope| {
                scope.spawn(move || {
                    for element in part {
                        transform_fn(element);
                    }
                });
            });
        }
    }

}
impl <T> Vertices<T>{
    pub fn new() -> Self {
        return Vertices{
            data: Vec::new(),
        }
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


pub struct EdgeData{
    edge_capacity: usize,
    edges: Vec<usize>,
    indices: Vec<usize>,
}

impl EdgeData {

    pub fn new() -> Self {
        return EdgeData{
            edge_capacity: 50,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        return EdgeData{
            edge_capacity: capacity,
            edges: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn add_edges(&mut self, vertex: usize, edges: &[usize]) {
        let head_offset = self.indices[vertex];
        let head_index = &self.edges[head_offset];
        let chunk_len = *head_index;
        let new_size = chunk_len + edges.len();

        if new_size > self.edge_capacity {
            panic!("Edge array full!");
        }

        if new_size > self.edges.len() {
            panic!("Edge size is greater than the allocated size");
        }

        let new_data_start = head_offset + chunk_len + header_element_size();
        let new_data_end = new_data_start + edges.len();





        self.edges[new_data_start..new_data_end].copy_from_slice(edges);
        self.edges[head_offset] = new_size;
    }

    #[cfg_attr(release, inline(always))]
    fn calculate_new_edges_size(&self) -> usize {
        return self.edges.len() + self.edge_capacity + (header_element_size());
    }
    fn create_vertex(&mut self) -> usize{
        let old_size = self.edges.len();

        self.edges.resize_with(self.calculate_new_edges_size(), Default::default);
        self.indices.push(old_size);
        return self.indices.len() - 1;
    }

    #[cfg_attr(release, inline(always))]
    pub fn edges_len<T>(&self, vertex: usize) -> usize {
        return self.edges[self.indices[vertex]];
    }

    #[cfg_attr(release, inline(always))]
    pub fn connect(&mut self, from: usize, to: usize) {
        self.add_edges(from, &[to]);
    }
    #[cfg_attr(release, inline(always))]
    pub fn len(&self) -> usize {
        return self.edges.len();
    }

    pub fn edges(&self, vertex: usize) -> Result< &[usize], Error> {
        let edge = self.indices[vertex];
        let size = self.edges[edge];

        if vertex > self.edges.len() {
            return Err(Error::NoHandle);
        }

        return Ok(&self.edges[edge + header_element_size()..edge + size + header_element_size() ]);
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

