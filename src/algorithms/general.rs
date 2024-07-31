use std::alloc::{alloc, dealloc, Layout};
use std::slice::{from_raw_parts_mut, Iter};
use firestorm::{profile_fn, profile_section};
use crate::graph;
use crate::handles::types::{VHandle, Weight};
use crate::handles::{Slot, vh};
use crate::traits::{EdgeStore, WeightedEdgeManipulate};
use crate::weighted_graph::WeightedGraph;

pub enum ControlFlow {
    Resume,
    End,
    Continue,
}


pub fn bfs<PreOrderFunc, Edges>(edge_storage: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order: PreOrderFunc)
where
    PreOrderFunc: FnMut(&mut Edges, VHandle, Weight) -> ControlFlow,
    Edges: EdgeStore
{
    profile_fn!(bfs);
    let to_visit_layout = Layout::array::<VHandle>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let flag_layout = Layout::array::<bool>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    let to_visit_ptr = unsafe {alloc(to_visit_layout)};
    let flags_ptr = unsafe {alloc(flag_layout)};
    unsafe {std::ptr::write_bytes(flags_ptr, 0, vertices_count)};

    let was_queued_flags = unsafe {from_raw_parts_mut(flags_ptr as *mut bool, vertices_count)};
    let to_visit = unsafe {from_raw_parts_mut(to_visit_ptr as *mut VHandle, vertices_count)};
    let mut end = 1;
    let mut next_layer = 1;
    let mut layer = 0;
    to_visit[0] = start;
    was_queued_flags[0] = true;
    let mut i = 0;

    while i != end {
        profile_section!(bfs_loop_outer);
        let handle = to_visit[i];
        match pre_order(edge_storage, handle, layer) { // the i is a place holder for the layer
            ControlFlow::End => {
                break;
            }
            ControlFlow::Continue => {
                i += 1;
                continue;
            }
            ControlFlow::Resume => {}
        }
        let edges = edge_storage.edges(handle);
        for next in edges {
            profile_section!(bfs_loop_inner);
            let next_handle = vh(*next);
            if was_queued_flags[next_handle as usize] {
                continue;
            }
            was_queued_flags[next_handle as usize] = true;
            to_visit[end] = next_handle;
            end += 1;
        }
        i +=1;

        if i == next_layer {
            layer += 1;
            next_layer = end;
        }
    }

    unsafe {dealloc(to_visit_ptr, to_visit_layout)};
    unsafe {dealloc(flags_ptr, flag_layout)};
}
pub fn dfs<PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order_func: PreOrderFunc,
                                               mut post_order_func: PostOrderFunc)
where
    PreOrderFunc: FnMut(&mut Edges, VHandle) -> ControlFlow,
    PostOrderFunc: FnMut(&mut Edges, VHandle),
    Edges: EdgeStore
{
    profile_fn!(dfs);
    let layout = Layout::array::<(*const Slot, *const Slot, VHandle)>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let flag_layout = Layout::array::<bool>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    // Have to use unsafe as the borrow checker doesn't know that flags and edges don't overlap
    let visit_ptr = unsafe {alloc(layout)};
    let flags_ptr = unsafe {alloc(flag_layout)};
    unsafe {std::ptr::write_bytes(flags_ptr, 0, vertices_count)};


    let to_visit = unsafe { visit_ptr as *mut (*const Slot, *const Slot, VHandle)};
    let was_visited_flags = unsafe {from_raw_parts_mut(flags_ptr as *mut bool, vertices_count)};
    let mut top = 0;
    unsafe {
        *to_visit.offset(top) = (edge_storage.edges_ptr(start), edge_storage.edges_ptr(start).add(edge_storage.len(start) as usize), start);
    }
    match pre_order_func(edge_storage, start){
        ControlFlow::End => {
            unsafe {dealloc(visit_ptr, layout)};
            unsafe {dealloc(flags_ptr, flag_layout)};
            return;
        },
        _ => {}
    }

    while top >= 0{
        profile_section!(dfs_loop);
        let (ptr, end, vertex) = unsafe{*to_visit.offset(top)};
        if ptr == end{
            post_order_func(edge_storage, vertex);
            top -= 1;
            continue;
        }
        unsafe {
            *to_visit.offset(top) = (ptr.add(1), end, vertex); // Move to the next edge
        }

        let current_handle = vh(unsafe{*ptr});
        if was_visited_flags[current_handle as usize]{
            continue;
        }

        was_visited_flags[current_handle as usize] = true;
        match pre_order_func(edge_storage, current_handle){
            ControlFlow::End => {
                break;
            },
            ControlFlow::Continue => {
                continue;
            },
            ControlFlow::Resume => {}
        }

        unsafe {
            *to_visit.offset(top + 1) = (edge_storage.edges_ptr(current_handle), edge_storage.edges_ptr(current_handle).add(edge_storage.len(current_handle) as usize), current_handle);
        }
        top += 1;
    }
    unsafe {dealloc(visit_ptr, layout)};
    unsafe {dealloc(flags_ptr, flag_layout)};
}