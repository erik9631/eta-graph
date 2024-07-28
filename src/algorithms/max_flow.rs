use std::alloc::{alloc, Layout};
use std::iter::zip;
use std::thread;
use crate::handles::types::Weight;
use crate::traits::{StoreVertex, WeightedManipulate};
use crate::utils::{split_to_parts, split_to_parts_mut};
use crate::vertex_storage::VertexStorage;
use crate::weighted_graph::WeightedGraph;

pub struct DinicVertex<VertexType> {
    pub vertex: VertexType,
    pub level: Weight,
    pub flow: Weight,
    pub sub_sum: Weight,
}

pub fn clone_from_vertices_to_dinic_vertices_async<StoreVertexType, VertexType>(vertices: &StoreVertexType) -> Vec<DinicVertex<VertexType>>
where StoreVertexType: StoreVertex<VertexType>, VertexType: Clone + Send + Sync {
    let layout = Layout::array::<DinicVertex<VertexType>>(vertices.len()).expect("Failed to create layout");
    let dinic_raw = unsafe {alloc(layout) as *mut DinicVertex<VertexType>};
    let mut dinic_vec = unsafe {Vec::from_raw_parts(dinic_raw, vertices.len(), vertices.len())};
    let mut dinic_parts = split_to_parts_mut(dinic_vec.as_mut_slice(), vertices.len());
    let vertex_parts = split_to_parts(&vertices.as_slice(), vertices.len());
    // Create iterators over the parts
    let dinic_iter = dinic_parts.iter_mut();
    let vertex_iter = vertex_parts.iter();

    // Zip the iterators
    let iter = zip(dinic_iter, vertex_iter);

    thread::scope(|s| {
        for (dinic_part, vertex_part) in iter {
            s.spawn(move || {
                for (dinic_vertex, vertex) in dinic_part.iter_mut().zip(vertex_part.iter()) {
                    *dinic_vertex = DinicVertex{
                        vertex: (*vertex).clone(),
                        level: 0,
                        flow: 0,
                        sub_sum: 0,
                    };
                }
            });
        }
    });

    return dinic_vec;
}

// pub fn hybrid_dinic<VertexType, EdgeStorageType>(graph: WeightedGraph<VertexType, EdgeStorageType>) -> WeightedGraph<VertexType, DinicVertex<VertexType>>
// where EdgeStorageType: WeightedManipulate {
//     let mut edges = graph.graph.edges.clone();
//     let mut vertices = clone_from_vertices_to_dinic_vertices(graph.graph.vertices.data);
// }
