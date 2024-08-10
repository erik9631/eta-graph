use crate::algorithms::general::bfs;
use crate::algorithms::general::ControlFlow::Resume;
use crate::algorithms::max_flow::DinicGraph;
use crate::handles::{vh, vh_pack, wgt};
use crate::traits::WeightedGraphOperate;
use crate::weighted_graph::WeightedGraph;

pub mod graph;
pub mod traits;
pub mod utils;
pub mod views;
pub mod edge_storage;
pub mod handles;
pub mod weighted_graph;
pub mod algorithms;
pub mod vertex_storage;

#[cfg(test)]
pub mod tests;
#[cfg(test)]
mod bench;
mod prelude;

pub fn dinic_test(){
    let mut graph = WeightedGraph::new();
    // Write test for layering
    let a = graph.graph.create("a", 2);
    let a_a = graph.create_and_connect_weighted(a, "a_a", 100, 1);
    let a_a_a = graph.create_and_connect_weighted(a_a, "a_a_a", 20, 1);
    let a_a_x = graph.create_and_connect_weighted(a_a_a, "a_a_x", 30, 0);

    let a_b = graph.create_and_connect_weighted(a, "a_b", 20, 3);
    let a_b_a = graph.create_and_connect_weighted(a_b, "a_b_a", 10, 1);
    let a_b_b = graph.create_and_connect_weighted(a_b, "a_b_b", 10, 1);
    let a_b_c = graph.create_and_connect_weighted(a_b, "a_b_c", 10, 1);

    graph.graph.edge_storage.connect_weighted(a_b_a, a_a_x, 10);
    graph.graph.edge_storage.connect_weighted(a_b_b, a_a_x, 10);
    graph.graph.edge_storage.connect_weighted(a_b_c, a_a_x, 10);

    let mut dinic_graph = DinicGraph::from(&graph.graph.vertices, &graph.graph.edge_storage);
    dinic_graph.perform_search(a, a_a_x);
    dinic_graph.finalize_flow_calc(&graph);

    let mut snap = vec![
        0,10,10,20,0,10,10,20,20,20,0
    ];

    bfs(&mut dinic_graph.edge_storage, vh_pack(a), dinic_graph.vertices.len(), |v_handle, layer|{
        println!("{} {}", dinic_graph.vertices[vh(*v_handle)], wgt(*v_handle));
        Resume
    });

}
fn main() {
    dinic_test();
}