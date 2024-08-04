use std::alloc::{alloc, dealloc, Layout};
use std::slice::{from_raw_parts_mut, Iter};
use firestorm::{profile_fn, profile_section};
use crate::graph;
use crate::handles::types::{PackedEdge, VHandle, Weight};
use crate::handles::{Slot, vh, vh_pack};
use crate::traits::{EdgeStore};

pub enum ControlFlow {
    Resume,
    End,
    Exit,
    Continue,
}


pub fn bfs<PreOrderFunc, Edges>(edge_storage: &mut Edges, start: VHandle, vertices_count: usize, mut pre_order: PreOrderFunc)
where
    PreOrderFunc: FnMut( &mut PackedEdge, Weight) -> ControlFlow,
    Edges: EdgeStore
{
    profile_fn!(bfs);
    let to_visit_layout = Layout::array::<*mut PackedEdge>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let flag_layout = Layout::array::<bool>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec

    let to_visit_ptr = unsafe {alloc(to_visit_layout)};
    let flags_ptr = unsafe {alloc(flag_layout)};
    unsafe {std::ptr::write_bytes(flags_ptr, 0, vertices_count)};

    let was_queued_flags = unsafe {from_raw_parts_mut(flags_ptr as *mut bool, vertices_count)};
    let to_visit = unsafe {from_raw_parts_mut(to_visit_ptr as *mut (*mut PackedEdge), vertices_count)};
    let mut end = 1;
    let mut next_layer = 1;
    let mut layer = 0;
    let mut start_edge = vh_pack(start);
    to_visit[0] = (&mut start_edge) as *mut PackedEdge;
    was_queued_flags[0] = true;
    let mut i = 0;

    while i != end {
        profile_section!(bfs_loop_outer);
        let handle = to_visit[i];
        match pre_order(unsafe {handle.as_mut().unwrap()}, layer) { // the i is a place holder for the layer
            ControlFlow::End => {
                break;
            }
            ControlFlow::Exit => {
                break;
            }
            ControlFlow::Continue => {
                i += 1;
                continue;
            }
            ControlFlow::Resume => {}
        }
        let mut edge = edge_storage.edges_mut_ptr(vh(unsafe{*handle}));
        let edges_end = unsafe { edge.add( edge_storage.len(vh(*handle)) as usize) };
        while edge != edges_end {
            profile_section!(bfs_loop_inner);
            if was_queued_flags[unsafe {vh(*edge)} as usize] {
                unsafe { edge = edge.add(1)};
                continue;
            }
            unsafe {was_queued_flags[vh(*edge) as usize] = true};
            to_visit[end] = edge;
            unsafe { edge = edge.add(1)};
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

pub fn alloc_flags(vertices_count: usize) -> (&'static mut[bool], Layout) {
    let flag_layout = Layout::array::<bool>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let flags_ptr = unsafe { alloc(flag_layout) };
    unsafe { std::ptr::write_bytes(flags_ptr, 0, vertices_count) };
    let flag_slice = unsafe {from_raw_parts_mut(flags_ptr as *mut bool, vertices_count)};
    (flag_slice, flag_layout)
}

pub fn dealloc_flags( flags: (&'static mut[bool], Layout) ) {
    unsafe { dealloc(unsafe{flags.0.as_mut_ptr() as *mut u8}, flags.1) };
}

pub fn reset_flags(flags: &mut[bool]) {
    unsafe { std::ptr::write_bytes(flags.as_mut_ptr(), 0, flags.len()) };
}


#[cfg_attr(not(debug_assertions), inline(always))]
pub fn dfs<PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: PackedEdge, vertices_count: usize, mut pre_order_func: PreOrderFunc,
                                               mut post_order_func: PostOrderFunc)
where
    PreOrderFunc: FnMut(&mut PackedEdge) -> ControlFlow,
    PostOrderFunc: FnMut(&mut PackedEdge),
    Edges: EdgeStore
{
    let flags = alloc_flags(vertices_count);
    dfs_custom_flags(edge_storage, start, flags.0, pre_order_func, post_order_func);
    dealloc_flags(flags);
}

// TODO Consider creating iter for the edges
pub fn dfs_custom_flags<PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: PackedEdge, visit_flags: &mut[bool] , mut pre_order_func: PreOrderFunc,
                                               mut post_order_func: PostOrderFunc)
where
    PreOrderFunc: FnMut(&mut PackedEdge) -> ControlFlow,
    PostOrderFunc: FnMut(&mut PackedEdge),
    Edges: EdgeStore
{
    profile_fn!(dfs);
    let layout = Layout::array::<(*const Slot, *const Slot, PackedEdge)>(visit_flags.len()).expect("Failed to create layout"); // Around ~50% faster than vec

    // Have to use unsafe as the borrow checker doesn't know that flags and edges don't overlap
    let visit_ptr = unsafe {alloc(layout)};

    let to_visit = unsafe { visit_ptr as *mut (*mut Slot, *mut Slot, *mut PackedEdge)};
    let stack = unsafe {from_raw_parts_mut(to_visit, visit_flags.len())};
    let mut top: isize = 0;
    let mut start_edge = start;
    unsafe {
        stack[top as usize] = (edge_storage.edges_mut_ptr(vh(start)), edge_storage.edges_mut_ptr(vh(start)).add(edge_storage.len(vh(start)) as usize), &mut start_edge as *mut PackedEdge);
    }
    match pre_order_func( unsafe {stack[top as usize].2.as_mut().unwrap() }){
        ControlFlow::End => {
            unsafe {dealloc(visit_ptr, layout)};
            return;
        },
        _ => {}
    }

    while top >= 0{
        profile_section!(dfs_loop);
        let (ptr, end, packed_edge) = stack[top as usize];
        if ptr == end{
            post_order_func( unsafe {packed_edge.as_mut().unwrap()});
            top -= 1;
            continue;
        }
        unsafe {
            stack[top as usize] = (ptr.add(1), end, packed_edge); // Move to the next edge
        }

        let next_handle = vh(unsafe{*ptr});
        let next_packed_edge = ptr;
        if visit_flags[next_handle as usize]{
            continue;
        }

        visit_flags[next_handle as usize] = true;
        match pre_order_func( unsafe{next_packed_edge.as_mut().unwrap()} ){
            ControlFlow::End => {
                break;
            },
            ControlFlow::Exit => {
                unsafe {dealloc(visit_ptr, layout)};
                return;;
            },
            ControlFlow::Continue => {
                continue;
            },
            ControlFlow::Resume => {}
        }

        unsafe {
            stack[ (top + 1) as usize] = (edge_storage.edges_mut_ptr(next_handle), edge_storage.edges_mut_ptr(next_handle).add(edge_storage.len(next_handle) as usize), next_packed_edge);
        }
        top += 1;
    }

    // Return back to the src without exploring further
    while top >= 0{
        profile_section!(dfs_post_loop);
        let (ptr, end, packed_edge) = stack[top as usize];
        post_order_func( unsafe{packed_edge.as_mut().unwrap()});
        top -= 1;
    }

    unsafe {dealloc(visit_ptr, layout)};
}