use crate::edge_storage::EdgeStorage;
use crate::handles::Slot;
use crate::handles::types::{PackedEdge, VHandle};
use crate::traits::EdgeOperator;

pub struct WeightedEdgeStorage {
    pub edge_storage: EdgeStorage,
    pub weight_indices: Vec<VHandle>,
    pub weights: Vec<VHandle>,
}

impl EdgeOperator for WeightedEdgeStorage {
    fn add_edges(&mut self, src: VHandle, targets: &[PackedEdge]) {

    }

    fn extend_edge_storage(&mut self, size: Slot) -> Slot {
        todo!()
    }

    fn disconnect(&mut self, src_handle: VHandle, handle: VHandle) {
        todo!()
    }

    fn connect(&mut self, from: VHandle, to: VHandle) {
        todo!()
    }
}