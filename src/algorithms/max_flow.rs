use std::alloc::{alloc, Layout};
use std::cell::{Cell};
use std::ptr;
use crate::algorithms::general::{alloc_flags, bfs, dealloc_flags, dfs, dfs_custom_flags, reset_flags};
use crate::algorithms::general::ControlFlow::{Continue, End, Resume};
use crate::handles::types::{VHandle, Weight};
use crate::handles::{set_wgt, vh, vh_pack_max, wgt};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
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
    VertexType: Clone + std::fmt::Display,
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
        bfs(&mut self.weighted_graph.graph.edge_storage, src_handle, self.weighted_graph.graph.vertices.len(), |v_handle, layer|{
            if vh(*v_handle) == sink_handle {
                found_sink = true;
            }

            if wgt(*v_handle) == 0 {
                return Continue;
            }

            self.flow_data[vh(*v_handle) as usize] = layer;
            Resume
        });

        if !found_sink {
            return Err("Sink not found");
        }
        return Ok(())
    }

    pub fn finalize_flow_calc(&mut self, original_graph: &WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>)
    where
        VertexType: Clone,
        VertexStorageType: StoreVertex<VertexType=VertexType> + Clone,
        EdgeStorageType: WeightedEdgeManipulate,
    {
        let zipped_iters = original_graph.graph.edge_storage.iter().zip(self.weighted_graph.graph.edge_storage.iter_mut());
        for (idx, edges) in zipped_iters.enumerate() {
            let (original_edge, dinic_edge) = edges;
            let original_wgt = unsafe { wgt(*original_edge) };
            let current_wgt = wgt(*dinic_edge);
            *dinic_edge = set_wgt(*dinic_edge, original_wgt - current_wgt);
        }
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        let mut generate_residual = true;
        let dfs_found_sink = Cell::new(true);
        let flags = alloc_flags(self.weighted_graph.graph.vertices.len());
        while generate_residual && dfs_found_sink.get() {
            if generate_residual {
                match self.mark_levels(src_handle, sink_handle) {
                    Ok(_) => {},
                    Err(_) => {
                        break;
                    }
                }
            }
            generate_residual = false;
            let bottleneck_value = Cell::new(Weight::MAX);
            loop {
                let mut last_layer = Cell::new(-1);
                bottleneck_value.set(Weight::MAX);
                dfs_found_sink.set(false);
                dfs_custom_flags(&mut self.weighted_graph.graph.edge_storage, vh_pack_max(src_handle), flags.0,
                                 |v_handle| {
                                     if dfs_found_sink.get() {
                                         return End;
                                     }

                                     if vh(*v_handle) == sink_handle {
                                        dfs_found_sink.set(true);
                                         return Resume;
                                     }

                                     if wgt(*v_handle) == 0 {
                                         return Continue;
                                     }

                                     if self.flow_data[vh(*v_handle) as usize] <= last_layer.get() {
                                         generate_residual = true;
                                         return Continue;
                                     }

                                     last_layer.set(self.flow_data[vh(*v_handle) as usize]);

                                     let weight = wgt(*v_handle);
                                     if wgt(*v_handle) < bottleneck_value.get() {
                                         bottleneck_value.set(weight);
                                     }
                                     Resume
                                 }, |v_handle|
                                 {
                                     last_layer.set(last_layer.get()-1);
                                     if !dfs_found_sink.get(){
                                         return;
                                     }
                                     *v_handle = set_wgt(*v_handle, wgt(*v_handle) - bottleneck_value.get());
                                    // println!("{} {}", vh(*v_handle), wgt(*v_handle));
                                 });
                // Only the root and sink is visited multiple times
                reset_flags(flags.0);
                if !dfs_found_sink.get(){
                    break;
                }
            }
        }
        dealloc_flags(flags);
    }
}
// pub fn hybrid_dinic<VertexType, VertexStorageType, EdgeStorageType>(graph: WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> DinicGraphView<VertexType, VertexStorageType, EdgeStorageType>
// where
//     EdgeStorageType: WeightedManipulate
// {
//
// }
