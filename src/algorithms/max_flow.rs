use std::alloc::{alloc, Layout};
use std::ptr;
use std::slice::{Iter};
use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::Resume;
use crate::graph::Error;
use crate::handles::types::{VHandle, Weight};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
use crate::vertex_storage::VertexStorage;
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

    pub fn mark_levels(&mut self, src_handle: VHandle, sink_handle: VHandle) -> Result<(), &str> {
        let mut found_sink = false;
        bfs(&mut self.weighted_graph.graph.edges, src_handle, self.weighted_graph.graph.vertices.len(), |edge_storage, v_handle, layer|{
            if v_handle == sink_handle {
                found_sink = true;
            }
            self.flow_data[v_handle as usize].level = layer;
            Resume
        });

        if !found_sink {
            return Err("Sink not found");
        }

        return Ok(())
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        let repeat_residual = false;
        let prev_v_handle = VHandle::MAX;
        let bottleneck_value = Weight::MAX;
        dfs(&mut self.weighted_graph.graph.edges, src_handle, self.flow_data.len(),
            |edge_storage, v_handle, stack, top| {
                Resume
        }, |edge_storage, v_handle, stack, top|
        {
        });
    }
    pub fn iter_zip(&self) -> std::iter::Zip<Iter<VertexType>, Iter<FlowData>> {
        self.weighted_graph.graph.vertices.iter().zip(self.flow_data.iter())
    }
}
// pub fn hybrid_dinic<VertexType, VertexStorageType, EdgeStorageType>(graph: WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> DinicGraphView<VertexType, VertexStorageType, EdgeStorageType>
// where
//     EdgeStorageType: WeightedManipulate
// {
//
// }
