use std::ops::{Index, IndexMut};
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::queue::Queue;
use eta_algorithms::data_structs::stack::Stack;
use crate::algorithms::general::dfs;
use crate::handles::types::{Edge, VHandle, Weight};
use crate::handles::{pack, set_wgt, vh, wgt};
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
            let original_wgt = unsafe { wgt(*original_edge) };
            let current_wgt = wgt(*dinic_edge);
            *dinic_edge = set_wgt(*dinic_edge, original_wgt - current_wgt);
        }
    }

    pub fn perform_search(&mut self, src_handle: VHandle, sink_handle: VHandle) {
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
            let mut dfs_search = 0;

            loop {
                dfs_search += 1;
                let len = self.edge_storage.vertex_len(src_handle);
                let current_edge_offset = self.edge_storage.vertex_index(src_handle);
                let mut current_edge = pack(src_handle, Weight::MAX);
                stack.push((current_edge_offset, current_edge_offset + len, (&mut current_edge) as *mut Edge));

                let mut augmented_path_found = false;
                let mut bottleneck_value = Weight::MAX;
                let mut current_layer = 0;

                while stack.len() > 0 {
                    let (current_edge_offset, end_offset, outgoing_edge) = stack.top_mut().unwrap();
                    let outgoing_edge_val = unsafe { **outgoing_edge };
                    current_layer = self.layer_data[vh(outgoing_edge_val) as usize];

                    if vh(outgoing_edge_val) == sink_handle {
                        augmented_path_found = true;
                    }

                    // In case of augmented path found, we need to backtrack and ignore everything else
                    if augmented_path_found {
                        unsafe {
                            let original = wgt(**outgoing_edge);
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
                    let next_edge_layer = self.layer_data[vh(next_edge) as usize];

                    *current_edge_offset += 1;

                    // Exploring deeper
                    if wgt(next_edge) != 0 && next_edge_layer > current_layer {
                        let next_edge_edges = self.edge_storage.vertex_index(vh(next_edge));
                        let next_edge_edges_end = next_edge_edges + self.edge_storage.vertex_len(vh(next_edge));
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
    src_handle: VHandle,
    sink_handle: VHandle,
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
    layer_data[src_handle as usize] = 0;

    while queue.len() > 0 {
        let handle_ptr = unsafe { queue.dequeue().unwrap() };
        let handle = unsafe { *handle_ptr };
        if vh(handle) == sink_handle {
            found_sink = true;
        }

        let len = edge_storage.vertex_len(vh(handle));
        let mut next_edge = edge_storage.vertex_as_mut_ptr(vh(handle));
        let edges_end = unsafe { next_edge.add(len as usize) };

        while next_edge != edges_end {

            let next_edge_layer = layer_data[vh(unsafe { *next_edge }) as usize];

            if next_edge_layer != Weight::MAX {
                unsafe { next_edge = next_edge.add(1) };
                continue;
            }

            if wgt(unsafe { *next_edge }) == 0 {
                unsafe { next_edge = next_edge.add(1) };
                continue;
            }

            unsafe {
                layer_data[vh(*next_edge) as usize] = layer + 1;
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

