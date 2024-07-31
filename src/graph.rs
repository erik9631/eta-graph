use crate::edge_storage::{EdgeStorage};
use crate::handles::Slot;
use crate::handles::types::{VHandle};
use crate::traits::{Operate, Manipulate, StoreVertex};
use crate::vertex_storage::VertexStorage;
use crate::views::tree::TreeView;

#[derive(Debug)]
pub enum Error {
    NoHandle,
}
pub struct Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    pub vertices: VertexStorageType,
    pub edges: EdgeStorageType,
}

impl<VertexType, VertexStorageType, EdgeStorageType> Clone for Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: Manipulate,
    VertexType: Clone,
    VertexStorageType: StoreVertex<VertexType=VertexType> + Clone{
    fn clone(&self) -> Self {
        return Graph{
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.clone_from(&source.vertices);
        self.edges.clone_from(&source.edges);
    }
}

impl<VertexType> Graph<VertexType, VertexStorage<VertexType>, EdgeStorage>
where
{
    pub fn new_large() -> Self {
        return Graph{
            edges: EdgeStorage::new_large(),
            vertices: VertexStorage::new(),
        }
    }
    pub fn with_reserve(reserve: Slot) -> Self {
        return Graph{
            edges: EdgeStorage::with_reserve(reserve),
            vertices: VertexStorage::new(),
        };
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. Small reserve count of 5
    pub fn new() -> Self {
        return Graph{
            edges: EdgeStorage::new(),
            vertices: VertexStorage::new(),
        };
    }


}

impl<VertexType, VertexStorageType, EdgeStorageType> Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: Manipulate,
    VertexStorageType: StoreVertex<VertexType=VertexType>{
    pub fn tree_view(&mut self) -> TreeView<VertexType, VertexStorageType, EdgeStorageType> {
        return TreeView::new(&mut self.edges, &mut self.vertices);
    }

    pub fn create_and_connect(&mut self, src_vertex: VHandle, val: VertexType, edge_count: Slot) -> VHandle {
        let new_vertex = self.create(val, edge_count);
        self.edges.connect(src_vertex, new_vertex);
        return new_vertex;
    }

    pub fn create_and_connect_leaf(&mut self, src_vertex: VHandle, val: VertexType) -> VHandle {
        return self.create_and_connect(src_vertex, val, 0);
    }

    pub fn create(&mut self, val: VertexType, edge_count: Slot) -> VHandle {
        self.vertices.push(val);
        let new_vertex = (self.vertices.len() - 1)  as VHandle;
        self.edges.extend_edge_storage(edge_count);
        return new_vertex;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_leaf(&mut self, val: VertexType) -> VHandle {
        return self.create(val, 0)
    }
}