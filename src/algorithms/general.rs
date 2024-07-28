use std::alloc::{alloc, dealloc, Layout};
use std::slice::{from_raw_parts_mut, Iter};
use firestorm::{profile_fn, profile_section};
use crate::graph;
use crate::handles::types::{VHandle, Weight};
use crate::handles::{Slot, vh};
use crate::traits::{Store, Visit, WeightedManipulate};
use crate::weighted_graph::WeightedGraph;

pub enum ControlFlow {
    Resume,
    End,
    Continue,
}


pub fn bfs<PreOrderFunc, Edges>(edge_storage: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order: PreOrderFunc)
where PreOrderFunc: FnMut(&mut Edges, VHandle) -> ControlFlow, Edges: Store + Visit
{
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
        match pre_order(edge_storage, handle) {
            ControlFlow::End => {
                edge_storage.inc_global_visited_flag();
                break;
            }
            ControlFlow::Continue => {
                i += 1;
                continue;
            }
            ControlFlow::Resume => {}
        }
        edge_storage.inc_visited_flag(handle);

        let edges = edge_storage.edges(handle);
        for next in edges {
            profile_section!(bfs_loop_inner);
            let handle = vh(*next);
            if edge_storage.visited_flag(handle) == edge_storage.global_visited_flag() {
                continue;
            }
            to_visit[end] = handle;
            end += 1;
        }
        i +=1;
    }

    edge_storage.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}
pub fn dfs<PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order_func: PreOrderFunc,
                                               mut post_order_func: PostOrderFunc)
where PreOrderFunc: FnMut(&mut Edges, VHandle) -> ControlFlow, PostOrderFunc: FnMut(&mut Edges, VHandle), Edges: Store + Visit
{
    profile_fn!(dfs);
    let layout = Layout::array::<(*const Slot, *const Slot, VHandle)>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    // Have to use unsafe as the borrow checker doesn't know that flags and edges don't overlap
    let memory_ptr = unsafe {alloc(layout)};
    let to_visit = unsafe {memory_ptr as *mut (*const Slot, *const Slot, VHandle)};
    let mut top = 0;
    unsafe {
        *to_visit.offset(top) = (edge_storage.edges_ptr(start), edge_storage.edges_ptr(start).add(edge_storage.len(start) as usize), start);
    }
    //Special case for the root:
    edge_storage.inc_visited_flag(start);
    match pre_order_func(edge_storage, start){
        ControlFlow::End => {
            edge_storage.inc_global_visited_flag();
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
        if edge_storage.visited_flag(current_handle) == edge_storage.global_visited_flag() {
            continue;
        }

        edge_storage.inc_visited_flag(current_handle);
        match pre_order_func(edge_storage, current_handle){
            ControlFlow::End => {
                edge_storage.inc_global_visited_flag();
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
    edge_storage.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
    unsafe {dealloc(memory_ptr, layout)};
}