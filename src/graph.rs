use crate::edge_storage::{EdgeStorage};
use crate::handles::Slot;
use crate::handles::types::{EHandle};
use crate::traits::{EdgeConnect, EdgeManipulate, StoreVertex};
use crate::vertex_storage::VertexStorage;
use crate::views::tree::Tree;

#[derive(Debug)]
pub enum Error {
    NoHandle,
}
pub struct Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: EdgeManipulate,
{
    pub vertices: VertexStorageType,
    pub edge_storage: EdgeStorageType,
}

impl<VertexType, VertexStorageType, EdgeStorageType> Clone for Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: EdgeManipulate,
    VertexType: Clone,
    VertexStorageType: StoreVertex<VertexType=VertexType> + Clone {
    fn clone(&self) -> Self {
        Graph{
            vertices: self.vertices.clone(),
            edge_storage: self.edge_storage.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.clone_from(&source.vertices);
        self.edge_storage.clone_from(&source.edge_storage);
    }
}

impl<VertexType> Default for Graph<VertexType, VertexStorage<VertexType>, EdgeStorage> {
    fn default() -> Self {
        Self::new()
    }
}

impl<VertexType> Graph<VertexType, VertexStorage<VertexType>, EdgeStorage>
{
    pub fn new_large() -> Self {
        Graph{
            edge_storage: EdgeStorage::new_large(),
            vertices: VertexStorage::new(),
        }
    }
    pub fn with_reserve(reserve: Slot) -> Self {
        Graph{
            edge_storage: EdgeStorage::with_reserve(reserve),
            vertices: VertexStorage::new(),
        }
    }
    pub fn new() -> Self {
        Graph{
            edge_storage: EdgeStorage::new(),
            vertices: VertexStorage::new(),
        }
    }


}

impl<VertexType, VertexStorageType, EdgeStorageType> Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: EdgeManipulate,
    VertexStorageType: StoreVertex<VertexType=VertexType>{
    pub fn tree_view(&mut self) -> Tree<VertexType, VertexStorageType, EdgeStorageType> {
        return Tree::new(&mut self.edge_storage, &mut self.vertices);
    }

    pub fn create_and_connect(&mut self, from: EHandle, val: VertexType, edge_count: Slot) -> EHandle {
        let new_vertex = self.create(val, edge_count);
        self.edge_storage.connect(from, new_vertex);
        new_vertex
    }

    pub fn create_and_connect_0(&mut self, from: EHandle, val: VertexType) -> EHandle {
        self.create_and_connect(from, val, 0)
    }

    pub fn create(&mut self, val: VertexType, edge_count: Slot) -> EHandle {
        self.vertices.push(val);
        self.edge_storage.create_entry(edge_count)
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_leaf(&mut self, val: VertexType) -> EHandle {
        self.create(val, 0)
    }
}