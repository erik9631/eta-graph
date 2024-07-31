use std::alloc::{alloc, Layout};
use std::ptr;
use std::slice::{Iter};
use crate::algorithms::general::bfs;
use crate::algorithms::general::ControlFlow::Resume;
use crate::handles::types::{VHandle, Weight};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
use crate::weighted_graph::WeightedGraph;

pub struct FlowData {
    pub level: Weight,
    pub flow: Weight,
    pub sub_sum: Weight,
}

pub struct DinicGraphView<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub weighted_graph: &'a mut WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>,
    pub flow_data: Vec<FlowData>,
}

impl<'a, VertexType, VertexStorageType, EdgeStorageType> DinicGraphView<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub fn from(graph: &'a mut WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> Self {
        let vertices_len = graph.graph.vertices.len();
        let layout = Layout::array::<FlowData>(vertices_len).expect("Failed to create layout");
        // TODO use SIMD
        let ptr = unsafe { alloc(layout) as *mut FlowData };
        unsafe {ptr::write_bytes(ptr, 0, vertices_len)};
        let flow_data = unsafe { Vec::from_raw_parts(ptr, vertices_len, vertices_len) };
        DinicGraphView {
            weighted_graph: graph,
            flow_data,
        }
    }

    pub fn mark_levels(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        let mut found_sink = false;
        bfs(&mut self.weighted_graph.graph.edges, src_handle, self.weighted_graph.graph.vertices.len(), |edge_storage, v_handle, layer|{
            if v_handle == sink_handle {
                found_sink = true;
            }
            self.flow_data[v_handle as usize].level = layer;
            self.flow_data[v_handle as usize].flow = 0;
            self.flow_data[v_handle as usize].sub_sum = 0;
            Resume
        })
    }

    pub fn iter_zip(&self) -> std::iter::Zip<Iter<VertexType>, Iter<FlowData>> {
        self.weighted_graph.graph.vertices.iter().zip(self.flow_data.iter())
    }
}
pub fn mark_levels() {
}

// pub fn hybrid_dinic<VertexType, EdgeStorageType>(graph: WeightedGraph<VertexType, EdgeStorageType>) -> WeightedGraph<DinicVertexStorage<VertexType>, EdgeStorageType>
// where EdgeStorageType: WeightedManipulate {
//     let mut edges = graph.graph.edges.clone();
//     let mut vertices = DinicVertexStorage::from(&graph.graph.vertices);
//     // let new_graph = Graph{
//     //     vertices,
//     //     edges,
//     // };
//
// }
