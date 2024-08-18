use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use eta_algorithms::data_structs::stack::Stack;
use crate::handles::types::{Edge, VHandle, Weight};
use crate::handles::{pack, set_wgt, vh, vhu, wgt};
use crate::traits::{StoreVertex, WeightedEdgeManipulate};
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
    VertexType: std::fmt::Debug + std::fmt::Display,
{
    pub fn from(vertices: &'a VertexStorageType, edge_storage: &EdgeStorageType, src_handle: VHandle, sink_handle: VHandle) -> Self {
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
        VertexType: std::fmt::Debug + std::fmt::Display,
    {
        let zipped_iters = original_edges.iter().zip(self.edge_storage.iter_mut());
        for edges in zipped_iters {
            let (original_edge, dinic_edge) = edges;
            let original_wgt = wgt(*original_edge);
            let current_wgt = wgt(*dinic_edge);
            *dinic_edge = set_wgt(*dinic_edge, original_wgt - current_wgt);
        }
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
        let mut stack = Stack::new(self.vertices.len());
        let mut queue = Queue::<VHandle>::new_pow2_sized(self.vertices.len()); // Direct pointer access is faster than offsets
        self.layer_data.fill(Weight::MAX);

        while mark_levels(src_handle, sink_handle, &mut self.edge_storage, &mut queue, &mut self.layer_data).is_ok() {
            loop {
                let start_edges = self.edge_storage.edges_as_mut_ptr(src_handle);
                let mut root_edge = pack(src_handle, Weight::MAX);
                stack.push((start_edges, (&mut root_edge) as *mut Edge));

                let mut augmented_path_found = false;
                let mut bottleneck_value = Weight::MAX;
                let mut current_layer;

                while !stack.is_empty() {
                    let (next_edges_ptr, edge_ptr) = stack.top_mut().unwrap();
                    let outgoing_edge_val = unsafe { **edge_ptr };
                    current_layer = self.layer_data[vh(outgoing_edge_val) as usize];

                    if vh(outgoing_edge_val) == sink_handle {
                        augmented_path_found = true;
                    }

                    // In case of augmented path found, we need to backtrack and ignore everything else
                    if augmented_path_found {
                        unsafe {
                            let modified_edge = set_wgt(**edge_ptr, wgt(**edge_ptr) - bottleneck_value);
                            (*edge_ptr).write(modified_edge);
                        };
                        stack.pop();
                        continue;
                    }

                    if wgt(outgoing_edge_val) < bottleneck_value {
                        bottleneck_value = wgt(outgoing_edge_val);
                    }

                    let next = next_edges_ptr.next();
                    // Backtracking
                    if next.is_none() {
                        stack.pop();
                        continue;
                    }
                    let next = next.unwrap();

                    let next_edges = self.edge_storage.edges_as_mut_ptr(vh(*next));
                    let next_edge_layer = self.layer_data[vh(*next) as usize];

                    // Exploring deeper
                    if wgt(*next) != 0 && next_edge_layer > current_layer {
                        stack.push((next_edges, next));
                    }
                }

                if !augmented_path_found {
                    break;
                }
            }
        }
    }
}

pub(in crate) fn mark_levels<EdgeStorageType>(
    src_handle: VHandle,
    sink_handle: VHandle,
    edge_storage: &mut EdgeStorageType,
    queue: &mut Queue<VHandle>,
    layer_data: &mut Array<Weight>,
) -> Result<(), &'static str>
where
    EdgeStorageType: WeightedEdgeManipulate,
{
    let mut found_sink = false;
    queue.push(src_handle);
    let mut layer = 0;

    let mut sibling_counter = 0;
    let mut last_sibling_in_layer = 1;
    let mut next_last_sibling_in_layer = 1;
    layer_data[src_handle as usize] = 0;

    while !queue.is_empty() {
        let v_handle = queue.dequeue().unwrap();
        if v_handle == sink_handle {
            found_sink = true;
        }

        for next_edge in edge_storage.edges_iter_mut(v_handle){
            let next_edge_layer = unsafe{*layer_data.index_unchecked(vhu(*next_edge))};
            if next_edge_layer != Weight::MAX {
                continue;
            }

            if wgt(*next_edge) == 0 {
                continue;
            }
            layer_data[vh(*next_edge) as usize] = layer + 1;

            queue.push(vh(*next_edge));
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

