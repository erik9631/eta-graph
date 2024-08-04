use crate::edge_storage::{EdgeStorage};
use crate::handles::Slot;
use crate::handles::types::{VHandle};
use crate::traits::{GraphOperate, EdgeManipulate, StoreVertex};
use crate::vertex_storage::VertexStorage;
use crate::views::tree::TreeView;

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
        return Graph{
            vertices: self.vertices.clone(),
            edge_storage: self.edge_storage.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.clone_from(&source.vertices);
        self.edge_storage.clone_from(&source.edge_storage);
    }
}

impl<VertexType> Graph<VertexType, VertexStorage<VertexType>, EdgeStorage>
{
    pub fn new_large() -> Self {
        return Graph{
            edge_storage: EdgeStorage::new_large(),
            vertices: VertexStorage::new(),
        }
    }
    pub fn with_reserve(reserve: Slot) -> Self {
        return Graph{
            edge_storage: EdgeStorage::with_reserve(reserve),
            vertices: VertexStorage::new(),
        };
    }

    /// Creates a new graph with the assumption that the graph size is known ahead of time. Small reserve count of 5
    pub fn new() -> Self {
        return Graph{
            edge_storage: EdgeStorage::new(),
            vertices: VertexStorage::new(),
        };
    }


}

impl<VertexType, VertexStorageType, EdgeStorageType> Graph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: EdgeManipulate,
    VertexStorageType: StoreVertex<VertexType=VertexType>{
    pub fn tree_view(&mut self) -> TreeView<VertexType, VertexStorageType, EdgeStorageType> {
        return TreeView::new(&mut self.edge_storage, &mut self.vertices);
    }

    pub fn create_and_connect(&mut self, from: VHandle, val: VertexType, edge_count: Slot) -> VHandle {
        let new_vertex = self.create(val, edge_count);
        self.edge_storage.connect(from, new_vertex);
        return new_vertex;
    }

    pub fn create_and_connect_leaf(&mut self, from: VHandle, val: VertexType) -> VHandle {
        return self.create_and_connect(from, val, 0);
    }

    pub fn create(&mut self, val: VertexType, edge_count: Slot) -> VHandle {
        self.vertices.push(val);
        let new_vertex = (self.vertices.len() - 1)  as VHandle;
        self.edge_storage.extend_edge_storage(edge_count);
        return new_vertex;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_leaf(&mut self, val: VertexType) -> VHandle {
        return self.create(val, 0)
    }
}