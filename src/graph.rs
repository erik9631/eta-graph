use std::cmp::min;
use std::ops::{Index, IndexMut};
use std::thread::available_parallelism;
use crate::edge_storage::{EdgeStorage};
use crate::handles::Slot;
use crate::handles::types::{VHandle, Weight};
use crate::traits;
use crate::traits::{EdgeOperator, WeightedEdgeOperator};
use crate::utils::{split_to_parts_mut};
use crate::views::tree::TreeView;

#[derive(Debug)]
pub enum Error {
    NoHandle,
}

pub struct Vertices<T> {
    data: Vec<T>,
}

pub struct Graph<T> {
    pub vertices: Vertices<T>,
    pub edges: EdgeStorage,
}


impl<T> Graph<T>{

    pub fn tree_view(&mut self) -> TreeView<T> {
        return TreeView::new(&mut self.edges, &mut self.vertices);
    }

    /// Creates a new graph with the assumption that the usage will be dynamic.
    /// It will create the graph with high reserve count of 50 to avoid reallocations.
    pub fn new_large() -> Self {
        return Graph{
            edges: EdgeStorage::new_dyn(),
            vertices: Vertices::new(),

        }
    }
    /// Creates a new graph with a custom reserve
    pub fn with_reserve(reserve: Slot) -> Self {
        return Graph{
            edges: EdgeStorage::with_reserve(reserve),
            vertices: Vertices::new(),
        };
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. Small reserve count of 5
    pub fn new() -> Self {
        return Graph{
            edges: EdgeStorage::new(),
            vertices: Vertices::new(),
        };
    }

    pub fn create_and_connect(&mut self, src_vertex: VHandle, val: T, edge_count: Slot) -> VHandle {
        let new_vertex = self.create(val, edge_count);
        self.edges.connect(src_vertex, new_vertex);
        return new_vertex;
    }

    pub fn create_and_connect_weighted(&mut self, src_vertex: VHandle, val: T, weight: Weight, edge_count: Slot) -> VHandle {
        let new_vertex = self.create(val, edge_count);
        self.edges.connect_weighted(src_vertex, new_vertex, weight);
        return new_vertex;
    }

    pub fn create_and_connect_leaf_weighted(&mut self, src_vertex: VHandle, val: T, weight: Weight) -> VHandle {
        return self.create_and_connect_weighted(src_vertex, val, weight, 0);
    }
    pub fn create_and_connect_leaf(&mut self, src_vertex: VHandle, val: T) -> VHandle {
        return self.create_and_connect(src_vertex, val, 0);
    }

    pub fn create(&mut self, val: T, edge_count: Slot) -> VHandle {
        self.vertices.push(val);
        let new_vertex = (self.vertices.len() - 1)  as VHandle;
        self.edges.extend_edge_storage(edge_count);
        return new_vertex;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_leaf(&mut self, val: T) -> VHandle {
        return self.create(val, 0)
    }
}


impl <T: Send> traits::Transformer<T> for Vertices<T> {
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

impl <T> Index<VHandle> for Vertices<T>{
    type Output = T;
    fn index(&self, index: VHandle) -> &Self::Output {
        return &self.data[index as usize];
    }
}

impl <T> IndexMut<VHandle> for Vertices<T>{
    fn index_mut(&mut self, index: VHandle) -> &mut Self::Output {
        return &mut self.data[index as usize];
    }
}