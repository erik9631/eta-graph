use std::alloc::{alloc, Layout};
use std::cell::{Cell, RefCell};
use std::ptr;
use std::slice::{Iter};
use crate::algorithms::general::{bfs, dfs};
use crate::algorithms::general::ControlFlow::{Continue, End, Resume};
use crate::graph::Error;
use crate::handles::types::{VHandle, Weight};
use crate::handles::{set_wgt, vh, wgt};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
use crate::vertex_storage::VertexStorage;
use crate::weighted_graph::WeightedGraph;

pub struct DinicGraph<VertexType, VertexStorageType, EdgeStorageType>
where
    VertexType: Clone,
    VertexStorageType: StoreVertex<VertexType=VertexType> + Clone,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub weighted_graph: WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>,
    pub flow_data: Vec<Weight>,
}

impl<VertexType, VertexStorageType, EdgeStorageType> DinicGraph<VertexType, VertexStorageType, EdgeStorageType>
where
    VertexType: Clone,
    VertexStorageType: StoreVertex<VertexType=VertexType> + Clone,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub fn from(graph: & WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> Self {
        let vertices_len = graph.graph.vertices.len();
        let layout = Layout::array::<Weight>(vertices_len).expect("Failed to create layout");
        // TODO use SIMD
        let ptr = unsafe { alloc(layout) as *mut Weight };
        unsafe {ptr::write_bytes(ptr, 0, vertices_len)};
        let flow_data = unsafe { Vec::from_raw_parts(ptr, vertices_len, vertices_len) };
        DinicGraph {
            weighted_graph: graph.clone(),
            flow_data,
        }
    }

    pub fn mark_levels(&mut self, src_handle: VHandle, sink_handle: VHandle) -> Result<(), &str> {
        let mut found_sink = false;
        bfs(&mut self.weighted_graph.graph.edges, src_handle, self.weighted_graph.graph.vertices.len(), |edge_storage, v_handle, layer|{
            if v_handle == sink_handle {
                found_sink = true;
            }

            if wgt(v_handle) == 0 {
                return Continue;
            }

            self.flow_data[v_handle as usize] = layer;
            Resume
        });

        if !found_sink {
            return Err("Sink not found");
        }
        return Ok(())
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        let mut generate_residual = false;
        let mut last_layer = -1;
        let bottleneck_value = Cell::new(Weight::MAX);
        dfs(&mut self.weighted_graph.graph.edges, src_handle, self.weighted_graph.graph.vertices.len(),
            |v_handle| {
                if vh(*v_handle) == sink_handle {
                    return End;
                }

                if wgt(*v_handle) == 0 {
                    return Continue;
                }

                if last_layer <= self.flow_data[vh(*v_handle) as usize] {
                    generate_residual = true;
                    return Continue;
                }

                last_layer = self.flow_data[vh(*v_handle) as usize];

                let weight = wgt(*v_handle);
                if wgt(*v_handle) < bottleneck_value.get() {
                    bottleneck_value.set(weight);
                }
                Resume
            }, |v_handle|
            {
                set_wgt(*v_handle, wgt(*v_handle) - bottleneck_value.get());
            });
    }
}
// pub fn hybrid_dinic<VertexType, VertexStorageType, EdgeStorageType>(graph: WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> DinicGraphView<VertexType, VertexStorageType, EdgeStorageType>
// where
//     EdgeStorageType: WeightedManipulate
// {
//
// }
