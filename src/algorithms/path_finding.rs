use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::stack::Stack;
use crate::handles::types::{Edge, EHandle, Weight};
use crate::handles::{eh, wgt};
use crate::traits::EdgeStore;

struct MinHeapPair {
    pub vertex: EHandle,
    pub f_score: Weight,
}

#[derive(Clone, Copy)]
struct PathVertex {
    pub from: EHandle,
    pub f_score: Weight,
}

impl MinHeapPair {
    pub fn new(vertex: EHandle, f_score: Weight) -> Self {
        MinHeapPair {
            vertex,
            f_score,
        }
    }
}

impl Eq for MinHeapPair {}

impl PartialEq<Self> for MinHeapPair {
    fn eq(&self, other: &Self) -> bool {
        return self.f_score == other.f_score;
    }
}

impl PartialOrd<Self> for MinHeapPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for MinHeapPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

fn reconstruct_path(paths: &mut Array<PathVertex>, start: EHandle, goal: EHandle) -> Stack<EHandle> {
    let mut path = Stack::new(paths.capacity());
    let mut current = goal;
    path.push(current);
    while current != start {
        let prev = paths[current as usize].from;
        path.push(prev);
        current = prev;
    }
    return path;
}

/// A* algorithm
/// f_scores are sums of (distances + heuristic) from start to current vertex
/// h_scores are heuristic values from start to current vertex
/// g_scores sums of distances from start to current vertex. Not used purely in this implementation, but summed up to f_scores
///
// TODO Parallelization potential. Split the graph into multiple subgraphs, and run A* on each subgraph in parallel.
// Put together the resulting paths
pub fn a_star<Edges, Heuristic>(edge_storage: &mut Edges, start: EHandle, goal: EHandle, vertices_count: usize, h_score: Heuristic) -> Option<Stack<EHandle>>
where
    Edges: EdgeStore,
    Heuristic: Fn(EHandle, Edge) -> Weight,
{
    let mut explore_list = BinaryHeap::<MinHeapPair>::with_capacity(vertices_count);

    let mut f_scores = Array::<PathVertex>::new_with_default(vertices_count, PathVertex{from: 0, f_score: Weight::MAX});
    explore_list.push(MinHeapPair {vertex: start, f_score: 0});

    while let Some(current_vertex) = explore_list.pop() {
        if current_vertex.vertex == goal{
            return Some(reconstruct_path(&mut f_scores, start, goal))
        }

        let neighbors = edge_storage.entry_as_slice(current_vertex.vertex);
        for neighbor in neighbors {
            let neighbor_f_score = wgt(*neighbor) + current_vertex.f_score + h_score(current_vertex.vertex, *neighbor);
            if f_scores[eh(*neighbor) as usize].f_score < neighbor_f_score {
                continue;
            }
            explore_list.push(MinHeapPair::new(eh(*neighbor), neighbor_f_score));
            f_scores[eh(*neighbor) as usize] = PathVertex{from: current_vertex.vertex, f_score: neighbor_f_score };
        }
    }
    None
}

#[inline(always)]
pub fn dijkstra<Edges>(edge_storage: &mut Edges, start: EHandle, goal: EHandle, vertices_count: usize) -> Option<Stack<EHandle>>
where
    Edges: EdgeStore
{
    return a_star(edge_storage, start, goal, vertices_count, |from, to| {
        return 0;
    })
}