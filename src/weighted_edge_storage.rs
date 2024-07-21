use crate::edge_storage::EdgeStorage;
use crate::size::MSize;

pub struct WeightedEdgeStorage {
    pub edge_storage: EdgeStorage,
    pub weight_indices: Vec<MSize>,
    pub weights: Vec<MSize>,
}