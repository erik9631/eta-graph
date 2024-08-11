use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::stack::Stack;
use crate::handles::types::{Edge, VHandle, Weight};
use crate::handles::{vh, wgt};
use crate::traits::EdgeStore;

struct HeapPair{
    pub vertex: VHandle,
    pub f_score: Weight,
}

#[derive(Clone, Copy)]
struct PathVertex {
    pub from: VHandle,
    pub distance: Weight,
}

impl HeapPair {
    pub fn new(vertex: VHandle, f_score: Weight) -> Self {
        HeapPair {
            vertex,
            f_score,
        }
    }
}

impl Eq for HeapPair {}

impl PartialEq<Self> for HeapPair {
    fn eq(&self, other: &Self) -> bool {
        return self.f_score == other.f_score;
    }
}

impl PartialOrd<Self> for HeapPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for HeapPair{
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

fn reconstruct_path(paths: &mut Array<PathVertex>, start: VHandle, goal: VHandle) -> Stack<VHandle> {
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
pub fn a_star<Edges, Heuristic>(edge_storage: &mut Edges, start: VHandle, goal: VHandle, vertices_count: usize, h_score: Heuristic) -> Option<Stack<VHandle>>
where
    Edges: EdgeStore,
    Heuristic: Fn(VHandle, Edge) -> Weight,
{
    let mut explore_list = BinaryHeap::<HeapPair>::with_capacity(vertices_count);

    let mut f_scores = Array::<PathVertex>::new_with_default(vertices_count, PathVertex{from: 0, distance: Weight::MAX});
    explore_list.push(HeapPair{vertex: start, f_score: 0});

    while let Some(current_vertex) = explore_list.pop() {
        if current_vertex.vertex == goal{
            return Some(reconstruct_path(&mut f_scores, start, goal))
        }

        let neighbors = edge_storage.edges(current_vertex.vertex);
        for neighbor in neighbors {
            let neighbor_f_score = wgt(*neighbor) + current_vertex.f_score + h_score(current_vertex.vertex, *neighbor);
            if f_scores[vh(*neighbor) as usize].distance < neighbor_f_score {
                continue;
            }
            explore_list.push(HeapPair::new(vh(*neighbor), neighbor_f_score));
            f_scores[vh(*neighbor) as usize] = PathVertex{from: current_vertex.vertex, distance: neighbor_f_score };
        }
    }
    None
}

#[inline(always)]
pub fn dijkstra<Edges>(edge_storage: &mut Edges, start: VHandle, goal: VHandle, vertices_count: usize) -> Option<Stack<VHandle>>
where
    Edges: EdgeStore
{
    return a_star(edge_storage, start, goal, vertices_count, |from, to| {
        return 0;
    })
}