use crate::edge_storage::EdgeStorage;
use crate::graph::{Graph};
use crate::handles::Slot;
use crate::handles::types::{VHandle, Weight};
use crate::traits::{StoreVertex, WeightedEdgeManipulate, WeightedGraphOperate};
use crate::vertex_storage::VertexStorage;

pub struct WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: WeightedEdgeManipulate,
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    pub graph: Graph<VertexType, VertexStorageType, EdgeStorageType>,
}

impl<VertexType, VertexStorageType, EdgeStorageType> Clone for WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>
where
    EdgeStorageType: WeightedEdgeManipulate,
    VertexType: Clone,
    VertexStorageType: StoreVertex<VertexType=VertexType> + Clone {
    fn clone(&self) -> Self {
        return WeightedGraph{
            graph: self.graph.clone(),
        }
    }
}

impl<VertexType> WeightedGraph<VertexType, VertexStorage<VertexType>, EdgeStorage> {
    pub fn new() -> Self {
        return WeightedGraph{
            graph: Graph::new(),
        }
    }
    pub fn new_large() -> Self {
        return WeightedGraph{
            graph: Graph::new_large(),
        }
    }
    pub fn with_reserve(reserve: Slot) -> Self {
        return WeightedGraph{
            graph: Graph::with_reserve(reserve),
        }
    }
}
impl<VertexType, StoreVertexType, EdgeStorageType> WeightedGraph<VertexType, StoreVertexType, EdgeStorageType>
where
    EdgeStorageType: WeightedEdgeManipulate,
    StoreVertexType: StoreVertex<VertexType=VertexType> {
    pub fn create_and_connect_weighted(&mut self, src_vertex: VHandle, val: VertexType, weight: Weight, edge_count: Slot) -> VHandle {
        let new_vertex = self.graph.create(val, edge_count);
        self.graph.edges.connect_weighted(src_vertex, new_vertex, weight);
        return new_vertex;
    }

    pub fn create_and_connect_leaf_weighted(&mut self, src_vertex: VHandle, val: VertexType, weight: Weight) -> VHandle {
        return self.create_and_connect_weighted(src_vertex, val, weight, 0);
    }

}