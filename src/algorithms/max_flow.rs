use std::ops::{Index, IndexMut};
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use eta_algorithms::data_structs::stack::Stack;
use crate::handles::types::{Edge, EHandle, Weight};
use crate::handles::{pack, set_wgt, eh, wgt};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
const DUMMY_WEIGHT: Weight = -1;


pub struct DinicGraph<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub vertices: &'a VertexStorageType,
    pub edge_storage: EdgeStorageType,
    pub layer_data: Array<Weight>,
}

impl<'a, VertexType, VertexStorageType, EdgeStorageType> DinicGraph<'a, VertexType, VertexStorageType, EdgeStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: WeightedEdgeManipulate,
{
    pub fn from(vertices: &'a VertexStorageType, edge_storage: &EdgeStorageType, src_handle: EHandle, sink_handle: EHandle) -> Self {
        let vertices_len = vertices.len();
        let mut dinic_graph = DinicGraph {
            vertices,
            edge_storage: edge_storage.clone(),
            layer_data: Array::new(vertices_len),
        };

        dinic_graph.perform_search(src_handle, sink_handle);
        dinic_graph.finalize_flow_calc(edge_storage);
        dinic_graph
    }

    fn finalize_flow_calc(&mut self, original_edges: &EdgeStorageType)
    where
        VertexStorageType: StoreVertex<VertexType=VertexType>,
        EdgeStorageType: WeightedEdgeManipulate,
    {
        let zipped_iters = original_edges.iter().zip(self.edge_storage.iter_mut());
        for edges in zipped_iters {
            let (original_edge, dinic_edge) = edges;
            let original_wgt = unsafe { wgt(*original_edge) };
            let current_wgt = wgt(*dinic_edge);
            *dinic_edge = set_wgt(*dinic_edge, original_wgt - current_wgt);
        }
    }

    pub fn perform_search(&mut self, src_handle: EHandle, sink_handle: EHandle) {
        let mut stack = Stack::new(self.vertices.len());
        let mut queue = Queue::<*mut Edge>::new_pow2_sized(self.vertices.len()); // Direct pointer access is faster than offsets
        self.layer_data.fill(Weight::MAX);

        loop {
            match mark_levels(src_handle, sink_handle, &mut self.edge_storage, &mut queue,&mut self.layer_data) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }

            loop {
                let len = self.edge_storage.entry_len(src_handle);
                let current_edge_offset = self.edge_storage.entry_index(src_handle);
                let mut current_edge = pack(src_handle, Weight::MAX);
                stack.push((current_edge_offset, current_edge_offset + len, (&mut current_edge) as *mut Edge));

                let mut augmented_path_found = false;
                let mut bottleneck_value = Weight::MAX;
                let mut current_layer = 0;

                while stack.len() > 0 {
                    let (current_edge_offset, end_offset, outgoing_edge) = stack.top_mut().unwrap();
                    let outgoing_edge_val = unsafe { **outgoing_edge };
                    current_layer = self.layer_data[eh(outgoing_edge_val) as usize];

                    if eh(outgoing_edge_val) == sink_handle {
                        augmented_path_found = true;
                    }

                    // In case of augmented path found, we need to backtrack and ignore everything else
                    if augmented_path_found {
                        unsafe {
                            let modified_edge = set_wgt(**outgoing_edge, wgt(**outgoing_edge) - bottleneck_value);
                            (*outgoing_edge).write(modified_edge);
                        };
                        stack.pop();
                        continue;
                    }

                    if wgt(outgoing_edge_val) < bottleneck_value {
                        bottleneck_value = wgt(outgoing_edge_val);
                    }

                    // Backtracking
                    if *current_edge_offset == *end_offset {
                        stack.pop();
                        continue;
                    }

                    let next_edge_ptr = &mut self.edge_storage[*current_edge_offset] as *mut Edge;
                    let next_edge = unsafe { *next_edge_ptr };
                    let next_edge_layer = self.layer_data[eh(next_edge) as usize];

                    *current_edge_offset += 1;

                    // Exploring deeper
                    if wgt(next_edge) != 0 && next_edge_layer > current_layer {
                        let next_edge_edges = self.edge_storage.entry_index(eh(next_edge));
                        let next_edge_edges_end = next_edge_edges + self.edge_storage.entry_len(eh(next_edge));
                        current_layer = next_edge_layer;
                        stack.push((next_edge_edges, next_edge_edges_end, next_edge_ptr));
                    }
                }

                if !augmented_path_found {
                    break;
                }
            }
        }
    }
}

pub(in crate) fn mark_levels<EdgeStorageType, LayerDataType>(
    src_handle: EHandle,
    sink_handle: EHandle,
    edge_storage: &mut EdgeStorageType,
    queue: &mut Queue<*mut Edge>,
    layer_data: &mut LayerDataType,
) -> Result<(), &'static str>
where
    EdgeStorageType: WeightedEdgeManipulate,
    LayerDataType: Index<usize, Output=Weight> + IndexMut<usize, Output=Weight>,
{
    let mut found_sink = false;
    let mut start = pack(src_handle, DUMMY_WEIGHT);
    queue.push(&mut start as *mut Edge);
    let mut layer = 0;

    let mut sibling_counter = 0;
    let mut last_sibling_in_layer = 1;
    let mut next_last_sibling_in_layer = 1;

    while queue.len() > 0 {
        let handle_ptr = unsafe { queue.dequeue().unwrap() };
        let handle = unsafe { *handle_ptr };
        if eh(handle) == sink_handle {
            found_sink = true;
        }
        layer_data[eh(handle) as usize] = layer;

        let len = edge_storage.entry_len(eh(handle));
        let mut next_edge = edge_storage.entry_as_mut_ptr(eh(handle));
        let edges_end = unsafe { next_edge.add(len as usize) };

        while next_edge != edges_end {
            if layer_data[eh(unsafe { *next_edge }) as usize] <= layer {
                continue;
            }

            if wgt(unsafe { *next_edge }) == 0 {
                unsafe { next_edge = next_edge.add(1) };
                continue;
            }
            queue.push(next_edge);
            unsafe { next_edge = next_edge.add(1) };
            next_last_sibling_in_layer += 1;
        }
        sibling_counter += 1;
        if sibling_counter == last_sibling_in_layer {
            last_sibling_in_layer = next_last_sibling_in_layer;
            layer += 1;
        }
    }

    if !found_sink {
        return Err("Sink not found");
    }
    Ok(())
}
