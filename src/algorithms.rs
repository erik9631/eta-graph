use std::alloc::{alloc, dealloc, Layout};
use std::slice::{from_raw_parts_mut, Iter};
use firestorm::{profile_fn, profile_section};
use crate::graph::TraverseResult;
use crate::graph::TraverseResult::End;
use crate::handles::types::{VHandle};
use crate::handles::{Slot, vh};
use crate::traits::{EdgeStore, TraverseMarker};

pub fn bfs<PreOrderFunc, Edges>(edges: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order: PreOrderFunc) where
    PreOrderFunc: FnMut(&mut Edges, VHandle) -> TraverseResult,
    Edges: EdgeStore + TraverseMarker{
    profile_fn!(bfs);
    let layout = Layout::array::<VHandle>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let memory_ptr = unsafe {alloc(layout)};
    let to_visit = unsafe {from_raw_parts_mut(memory_ptr as *mut VHandle, vertices_count)};
    let mut end = 1;
    to_visit[0] = start;
    let mut i = 0;

    while i != end {
        profile_section!(bfs_loop_outer);
        let handle = to_visit[i];
        if pre_order(edges, handle) == End{
            edges.inc_global_visited_flag();
            break;
        }
        edges.inc_visited_flag(handle);

        let edges = edges.edges(handle);
        for next in edges {
            profile_section!(bfs_loop_inner);
            let handle = vh(*next);
            if edges.visited_flag(handle) == edges.global_visited_flag() {
                continue;
            }
            to_visit[end] = handle;
            end += 1;
        }
        i +=1;
    }

    edges.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}
pub fn dfs<PreOrderFunc, PostOrderFunc, Edges>(edges: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order_func: PreOrderFunc, mut post_order_func: PostOrderFunc) where
    PreOrderFunc: FnMut(&mut Edges, VHandle) -> TraverseResult,
    PostOrderFunc: FnMut(&mut Edges, VHandle),
    Edges: EdgeStore + TraverseMarker{
    profile_fn!(dfs);
    let layout = Layout::array::<(*const Slot, *const Slot, VHandle)>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    // Have to use unsafe as the borrow checker doesn't know that flags and edges don't overlap
    let memory_ptr = unsafe {alloc(layout)};
    let to_visit = unsafe {memory_ptr as *mut (*const Slot, *const Slot, VHandle)};
    let mut top = 0;
    unsafe {
        *to_visit.offset(top) = (edges.edges_ptr(start), edges.edges_ptr(start).add(edges.len(start) as usize), start);
    }
    //Special case for the root:
    edges.inc_visited_flag(start);
    if pre_order_func(edges, start) == End{
        edges.inc_global_visited_flag();
        return;
    }

    while top >= 0{
        profile_section!(dfs_loop);
        let (ptr, end, vertex) = unsafe{*to_visit.offset(top)};
        if ptr == end{
            post_order_func(edges, vertex);
            top -= 1;
            continue;
        }
        unsafe {
            *to_visit.offset(top) = (ptr.add(1), end, vertex); // Move to the next edge
        }

        let current_handle = vh(unsafe{*ptr});
        if edges.visited_flag(current_handle) == edges.global_visited_flag() {
            continue;
        }

        edges.inc_visited_flag(current_handle);
        if pre_order_func(edges, current_handle) == End{
            edges.inc_global_visited_flag();
            break;
        }
        unsafe {
            *to_visit.offset(top + 1) = (edges.edges_ptr(current_handle), edges.edges_ptr(current_handle).add(edges.len(current_handle) as usize), current_handle);
        }
        top += 1;
    }
    edges.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}
