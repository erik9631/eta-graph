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