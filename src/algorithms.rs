use std::alloc::{alloc, dealloc, Layout};
use std::slice::{from_raw_parts_mut, Iter};
use firestorm::profile_method;
use crate::graph::TraverseResult;
use crate::graph::TraverseResult::End;
use crate::handles::types::{PackedEdge, VHandle};
use crate::handles::{Slot, vh, vh_pack};
use crate::traits::{EdgeStore, TraverseMarker};

pub fn bfs<TraverseFunc, GraphType>(graph: &mut GraphType, start: VHandle, vertices_count: usize, mut transform: TraverseFunc) where
        TraverseFunc: FnMut(&mut GraphType, VHandle) -> TraverseResult,
        GraphType: EdgeStore + TraverseMarker{
    profile_method!(bfs);
    let layout = Layout::array::<VHandle>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let memory_ptr = unsafe {alloc(layout)};
    let to_visit = unsafe {from_raw_parts_mut(memory_ptr as *mut VHandle, vertices_count)};
    let mut end = 1;
    to_visit[0] = start;
    let mut i = 0;
    while i != end {
        let handle = to_visit[i];
        if transform(graph, handle) == End{
            graph.inc_global_visited_flag();
            unsafe {dealloc(memory_ptr, layout)};
            break;
        }
        graph.inc_visited_flag(handle);

        let edges = graph.edges(handle);
        for next in edges {
            if graph.visited_flag(vh(*next)) == graph.global_visited_flag() {
                continue;
            }
            to_visit[end] = vh(*next);
            end += 1;
        }
        i +=1;
    }
    graph.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}
pub fn dfs<TraverseFunc, GraphType>(graph: &mut GraphType, start: VHandle, vertices_count: usize, mut transform: TraverseFunc) where
        TraverseFunc: FnMut(&mut GraphType, VHandle) -> TraverseResult,
    GraphType: EdgeStore + TraverseMarker{
    profile_method!(dfs);
    let layout = Layout::array::<(*const PackedEdge, *const Slot)>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    // Have to use unsafe as the borrow checker doesn't know that flags and edges don't overlap
    let memory_ptr = unsafe {alloc(layout)};
    let to_visit = unsafe {memory_ptr as *mut (*const PackedEdge, *const Slot)};
    let mut top = 0;
    let start = [vh_pack(start);1 ];
    unsafe {
        *to_visit.offset(top) = (start.as_ptr(), start.as_ptr().add(1));
    }

    while top >= 0{
        let (ptr, end) = unsafe{*to_visit.offset(top)};
        unsafe {
            *to_visit.offset(top) = (ptr.add(1), end);
        }
        if ptr == end{
            top -= 1;
            continue;
        }
        let current_handle = vh(unsafe{*ptr});

        if graph.visited_flag(current_handle) == graph.global_visited_flag() {
            continue;
        }

        graph.inc_visited_flag(current_handle);
        if transform(graph, current_handle) == End{
            graph.inc_global_visited_flag();
            unsafe {dealloc(memory_ptr, layout)};
            return;
        }
        unsafe {
            *to_visit.offset(top + 1) = (graph.edges_ptr(current_handle), graph.edges_ptr(current_handle).add(graph.len(current_handle) as usize));
        }
        top += 1;
    }
    graph.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}
