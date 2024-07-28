use crate::edge_storage::EdgeStorage;
use crate::graph::{Graph, Vertices};
use crate::handles::Slot;
use crate::handles::types::{VHandle, Weight};
use crate::traits::{Operate, StoreMut, Visit, WeightedManipulate, WeightedOperate};

pub struct WeightedGraph<VertexType, EdgeStorageType> {
    pub graph: Graph<VertexType, EdgeStorageType>,
}

impl <VertexType> WeightedGraph<VertexType, EdgeStorage>{
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
impl<VertexType, EdgeStorageType> WeightedGraph<VertexType, EdgeStorageType>
where EdgeStorageType: WeightedManipulate
{
    pub fn create_and_connect_weighted(&mut self, src_vertex: VHandle, val: VertexType, weight: Weight, edge_count: Slot) -> VHandle {
        let new_vertex = self.graph.create(val, edge_count);
        self.graph.edges.connect_weighted(src_vertex, new_vertex, weight);
        return new_vertex;
    }

    pub fn create_and_connect_leaf_weighted(&mut self, src_vertex: VHandle, val: VertexType, weight: Weight) -> VHandle {
        return self.create_and_connect_weighted(src_vertex, val, weight, 0);
    }

}