use crate::handles::types::{Ci, Edge, VHandle, Weight};
use crate::handles::{vh, vh_pack};
use crate::traits::EdgeStore;
use eta_algorithms::data_structs::array::Array;
use eta_algorithms::data_structs::fat_ptr::{FatPtr, FatPtrMut};
use eta_algorithms::data_structs::queue::Queue;
use eta_algorithms::data_structs::stack::Stack;

pub enum ControlFlow {
    Resume,
    End,
    Exit,
    Continue,
}


pub fn bfs<PreOrderFunc, Edges>(edge_storage: &mut Edges, start: Edge, vertices_count: usize, mut pre_order: PreOrderFunc)
where
    PreOrderFunc: FnMut(&mut Edge, Weight) -> ControlFlow,
    Edges: EdgeStore,
{
    let mut was_queued_flags = Array::new_default_bytes(vertices_count, 0);

    // Uses more memory than necessary. But rotates very quickly. Might be worth considering version with smaller memory footprint.
    let mut visit_queue = Queue::<VHandle>::new_pow2_sized(vertices_count);
    let mut end = 1;
    let mut next_layer = 1;
    let mut layer = 0;
    visit_queue.push(vh(start));
    was_queued_flags[0] = true;
    let mut i = 0;

    //Initial call
    let mut start_edge = start;
    match pre_order(&mut start_edge, layer) {
        ControlFlow::End => {
            return;
        }
        ControlFlow::Exit => {
            return;
        }
        ControlFlow::Continue => {
        }
        ControlFlow::Resume => {}
    }

    while visit_queue.len() != 0 {
        let handle = visit_queue.dequeue().unwrap();

        for edge in edge_storage.edges_iter_mut(handle) {
            if unsafe { *was_queued_flags.index_unchecked(vh(*edge) as usize) } {
                continue;
            }
            unsafe { *was_queued_flags.index_unchecked_mut(vh(*edge) as usize) = true };

            match pre_order(edge, layer + 1) {
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

            visit_queue.push(vh(*edge));
            end += 1;
        }
        i += 1;

        if i == next_layer {
            layer += 1;
            next_layer = end;
        }
    }
}

#[cfg_attr(not(debug_assertions), inline(always))]
pub fn dfs<PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: Edge, vertices_count: usize, pre_order_func: PreOrderFunc,
                                               post_order_func: PostOrderFunc)
where
    PreOrderFunc: FnMut(&mut Edge) -> ControlFlow,
    PostOrderFunc: FnMut(&mut Edge),
    Edges: EdgeStore,
{
    let mut flags = Array::new_default_bytes(vertices_count, 0);
    dfs_custom_flags(edge_storage, start, vertices_count, |to_visit| {
        let was_visited = flags[vh(to_visit) as usize];
        flags[vh(to_visit) as usize] = true;
        was_visited
    }, pre_order_func, post_order_func);
}

pub fn dfs_custom_flags<VisitedFunc, PreOrderFunc, PostOrderFunc, Edges>(edge_storage: &mut Edges, start: Edge, vertex_count: usize,
                                                                         mut is_visited: VisitedFunc, mut pre_order_func: PreOrderFunc,
                                                                         mut post_order_func: PostOrderFunc)
where
    VisitedFunc: FnMut(Edge) -> bool,
    PreOrderFunc: FnMut(&mut Edge) -> ControlFlow,
    PostOrderFunc: FnMut(&mut Edge),
    Edges: EdgeStore,
{
    let mut start_edge = start;
    let mut stack = Stack::<(FatPtrMut<Edge>, *mut Edge)>::new(vertex_count);
    stack.push((edge_storage.edges_as_mut_ptr(vh(start)), (&mut start_edge) as *mut Edge));
    match pre_order_func(&mut start_edge) {
        ControlFlow::End => {
            return;
        }
        _ => {}
    }

    while stack.len() > 0 {
        let (outgoing_offset_iter, current_edge) = stack.top_mut().unwrap();
        let next = outgoing_offset_iter.next();
        if next.is_none() {
            post_order_func(unsafe { (*current_edge).as_mut().unwrap() });
            stack.pop();
            continue;
        }
        let next = next.unwrap();

        if is_visited(*next) {
            continue;
        }

        match pre_order_func(next) {
            ControlFlow::End => {
                break;
            }
            ControlFlow::Exit => {
                return;
            }
            ControlFlow::Continue => {
                continue;
            }
            ControlFlow::Resume => {}
        }

        let next_edges = edge_storage.edges_as_mut_ptr(vh(*next));
        stack.push((next_edges, next));
    }

    // Return back to the src without exploring further
    while stack.len() > 0 {
        let (_, packed_edge) = stack.pop().unwrap();
        post_order_func(unsafe { packed_edge.as_mut().unwrap() });
    }
}