use std::alloc::{alloc, Layout};
use std::cell::{Cell};
use std::ptr;
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use crate::algorithms::general::{ bfs, dfs_custom_flags};
use crate::algorithms::general::ControlFlow::{Continue, End, Resume};
use crate::edge_storage::EdgeStorage;
use crate::graph::Graph;
use crate::handles::types::{Edge, VHandle, Weight};
use crate::handles::{pack, set_wgt, vh, vh_pack, wgt};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
use crate::weighted_graph::WeightedGraph;

pub struct DinicGraph<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub vertices: &'a VertexStorageType,
    pub edge_storage: EdgeStorageType,
    pub flow_data: Vec<Weight>,
}

impl<'a, VertexType, VertexStorageType, EdgeStorageType> DinicGraph<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub fn from(vertices: &'a VertexStorageType, edge_storage: &EdgeStorageType) -> Self {
        let vertices_len = vertices.len();
        let layout = Layout::array::<Weight>(vertices_len).expect("Failed to create layout");
        // TODO use SIMD
        let ptr = unsafe { alloc(layout) as *mut Weight };
        unsafe { ptr::write_bytes(ptr, 0, vertices_len) };
        let flow_data = unsafe { Vec::from_raw_parts(ptr, vertices_len, vertices_len) };
        DinicGraph {
            vertices: &vertices,
            edge_storage: edge_storage.clone(),
            flow_data,
        }
    }

    pub fn mark_levels(&mut self, src_handle: VHandle, sink_handle: VHandle) -> Result<(), &str> {
        let mut found_sink = false;
        let mut start = pack(src_handle, -1);
        let mut queue = Queue::<*mut Edge>::new_pow2_sized(self.vertices.len()); // Direct pointer access is faster than offsets
        let mut visited_flag = Array::new_default_bytes(self.vertices.len(), 0);
        queue.push(&mut start as *mut Edge);
        let mut layer = 0;

        let mut sibling_counter = 0;
        let mut last_sibling_in_layer = 1;
        let mut next_last_sibling_in_layer = 1;

        while queue.len() > 0 {
            let handle_ptr = unsafe{queue.dequeue().unwrap()};
            let handle = unsafe{*handle_ptr};
            if vh(handle) == sink_handle {
                found_sink = true;
            }
            self.flow_data[vh(handle) as usize] = layer;

            let len = self.edge_storage.len(vh(handle));
            let mut next_edge = self.edge_storage.edges_mut_ptr(vh(handle));
            let edges_end = unsafe{ next_edge.add(len as usize)};
            if sibling_counter == last_sibling_in_layer {

            }

            while next_edge != edges_end {
                if visited_flag[vh(unsafe{*next_edge}) as usize] {
                    unsafe{ next_edge = next_edge.add(1)};
                    continue;
                }
                if wgt(unsafe{*next_edge}) == 0 {
                    unsafe{ next_edge = next_edge.add(1)};
                    continue;
                }


                visited_flag[vh(unsafe{*next_edge}) as usize] = true;
                queue.push(next_edge);
                unsafe{ next_edge = next_edge.add(1)};
                next_last_sibling_in_layer += 1;
            }
            sibling_counter += 1;
            if sibling_counter == last_sibling_in_layer{
                last_sibling_in_layer = next_last_sibling_in_layer;
                layer += 1;
            }
        }

        // bfs(&mut self.edge_storage, start, self.vertices.len(), |v_handle, layer| {
        //     if vh(*v_handle) == sink_handle {
        //         found_sink = true;
        //     }
        //
        //     if wgt(*v_handle) == 0 {
        //         return Continue;
        //     }
        //
        //     self.flow_data[vh(*v_handle) as usize] = layer;
        //     Resume
        // });

        if !found_sink {
            return Err("Sink not found");
        }
        return Ok(());
    }

    pub fn finalize_flow_calc(&mut self, original_graph: &WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>)
    where
        VertexStorageType: StoreVertex<VertexType=VertexType>,
        EdgeStorageType: WeightedEdgeManipulate,
    {
        let zipped_iters = original_graph.graph.edge_storage.iter().zip(self.edge_storage.iter_mut());
        for (idx, edges) in zipped_iters.enumerate() {
            let (original_edge, dinic_edge) = edges;
            let original_wgt = unsafe { wgt(*original_edge) };
            let current_wgt = wgt(*dinic_edge);
            *dinic_edge = set_wgt(*dinic_edge, original_wgt - current_wgt);
        }
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        loop {
            match self.mark_levels(src_handle, sink_handle) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            loop {
                let bottleneck_value = Cell::new(Weight::MAX);
                let mut last_layer = Cell::new(-1);
                bottleneck_value.set(Weight::MAX);
                let mut augmenting_path = Cell::new(false);
                dfs_custom_flags(&mut self.edge_storage,
                                 vh_pack(src_handle), self.vertices.len(), |edges| {
                        if last_layer.get() < self.flow_data[vh(edges) as usize] {
                            return false;
                        }
                        return true;
                    }, |v_handle| {
                        if vh(*v_handle) == sink_handle {
                            *v_handle = set_wgt(*v_handle, wgt(*v_handle) - bottleneck_value.get());
                            augmenting_path.set(true);
                            return End;
                        }

                        if wgt(*v_handle) == 0 {
                            return Continue;
                        }

                        let weight = wgt(*v_handle);
                        if wgt(*v_handle) < bottleneck_value.get() {
                            bottleneck_value.set(weight);
                        }
                        last_layer.set(self.flow_data[vh(*v_handle) as usize]);
                        Resume
                    }, |v_handle| {
                        last_layer.set(last_layer.get() - 1);
                        if !augmenting_path.get() {
                            return;
                        }
                        *v_handle = set_wgt(*v_handle, wgt(*v_handle) - bottleneck_value.get());
                    });
                if !augmenting_path.get() {
                    break;
                }
            }
        }
    }
}

// pub fn dinic<VertexType, VertexStorageType, EdgeStorageType>(graph: &mut WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>
// where
//     VertexType: Clone,
//
// {
//
// }
// pub fn hybrid_dinic<VertexType, VertexStorageType, EdgeStorageType>(graph: WeightedGraph<VertexType, VertexStorageType, EdgeStorageType>) -> DinicGraphView<VertexType, VertexStorageType, EdgeStorageType>
// where
//     EdgeStorageType: WeightedManipulate
// {
//
// }
