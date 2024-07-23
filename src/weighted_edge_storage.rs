use crate::edge_storage::EdgeStorage;
use crate::size::VHandle;

pub struct WeightedEdgeStorage {
    pub edge_storage: EdgeStorage,
    pub weight_indices: Vec<VHandle>,
    pub weights: Vec<VHandle>,
}

impl WeightedEdgeStorage {

}