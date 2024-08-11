use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::stack::Stack;
use crate::algorithms::general::ControlFlow::Resume;
use crate::handles::types::{VHandle, Weight};
use crate::handles::{vh, wgt};
use crate::traits::EdgeStore;

struct HeapPair{
    pub vertex: VHandle,
    pub distance: Weight,
}

#[derive(Clone, Copy)]
struct PathVertex {
    pub from: VHandle,
    pub distance: Weight,
}

impl HeapPair {
    pub fn new(vertex: VHandle, distance: Weight) -> Self {
        HeapPair {
            vertex,
            distance,
        }
    }
}

impl Eq for HeapPair {}

impl PartialEq<Self> for HeapPair {
    fn eq(&self, other: &Self) -> bool {
        return self.distance == other.distance;
    }
}

impl PartialOrd<Self> for HeapPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for HeapPair{
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
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
pub fn dijkstra<Edges>(edge_storage: &mut Edges, start: VHandle, goal: VHandle, vertices_count: usize) -> Option<Stack<VHandle>>
where
    Edges: EdgeStore
{
    let mut explore_list = BinaryHeap::<HeapPair>::with_capacity(vertices_count);
    let mut distances = Array::<PathVertex>::new_with_default(vertices_count, PathVertex{from: 0, distance: Weight::MAX});
    explore_list.push(HeapPair{vertex: start, distance: 0});

    while let Some(current_vertex) = explore_list.pop() {
        if current_vertex.vertex == goal{
            return Some(reconstruct_path(&mut distances, start, goal))
        }

        let neighbors = edge_storage.edges(current_vertex.vertex);
        for neighbor in neighbors {
            let new_distance= wgt(*neighbor) + current_vertex.distance;
            if distances[vh(*neighbor) as usize].distance < new_distance {
                continue;
            }
            explore_list.push(HeapPair::new(vh(*neighbor), wgt(*neighbor) + current_vertex.distance));
            distances[vh(*neighbor) as usize] = PathVertex{from: current_vertex.vertex, distance: new_distance};
        }
    }

    None
}