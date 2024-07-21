use std::alloc::{alloc, Layout};
use std::cmp::min;
use std::ops::{Index, IndexMut};
use std::slice::{from_raw_parts_mut};
use std::thread::available_parallelism;
use firestorm::{profile_method};
use crate::edge_data::{EdgeData};
use crate::graph::TraverseResult::End;
use crate::size::MSize;
use crate::traits;
use crate::traits::{GraphAccessor, TraverseMarker};
use crate::utils::{split_to_parts_mut};
use crate::views::tree::TreeView;

#[derive(Debug)]
pub enum Error {
    NoHandle,
}

#[derive(Eq, PartialEq)]
pub enum TraverseResult {
    Continue,
    End,
}


pub struct Vertices<T> {
    data: Vec<T>,
}

pub struct Graph<T> {
    pub vertices: Vertices<T>,
    pub edges: EdgeData,
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
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_leaf(&mut self, val: T) -> MSize {
        return self.create(val, 0)
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

    #[cfg_attr(not(debug_assertions), inline(always))]
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

