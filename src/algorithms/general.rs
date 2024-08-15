use crate::handles::types::{Ci, Edge, VHandle, Weight};
use crate::handles::{vh, vh_pack};
use crate::traits::EdgeStore;
use eta_algorithms::data_structs::array::Array;
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
    let mut was_queued_flags = Array::new_default_bytes(vertices_count, 0); // TODO Use a bit array for flags. Make such data structure for this.

    // Uses more memory than necessary. But rotates very quickly. Might be worth considering version with smaller memory footprint.
    let mut visit_queue = Queue::<VHandle>::new_pow2_sized(vertices_count); // TODO Use a bit queue
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

        for edge in edge_storage.vertex_iter_mut(handle) {
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
    let mut stack = Stack::<(usize, usize, *mut Edge)>::new(vertex_count);
    stack.push((edge_storage.vertex_index(vh(start)), edge_storage.vertex_index(vh(start)) + edge_storage.vertex_len(vh(start)), (&mut start_edge) as *mut Edge));
    match pre_order_func(&mut start_edge) {
        ControlFlow::End => {
            return;
        }
        _ => {}
    }

    while stack.len() > 0 {
        let (outgoing_offset_iter, end, current_edge) = stack.top_mut().unwrap();
        let outgoing_offset = *outgoing_offset_iter;
        if outgoing_offset_iter == end {
            post_order_func(unsafe { (*current_edge).as_mut().unwrap() });
            stack.pop();
            continue;
        }
        *outgoing_offset_iter += 1;

        let outgoing_edge = &mut edge_storage[outgoing_offset];
        if is_visited(*outgoing_edge) {
            continue;
        }

        match pre_order_func(outgoing_edge) {
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

        let outgoing_edge_edges_start = edge_storage.vertex_index(vh(edge_storage[outgoing_offset]));
        let outgoing_edge_edges_end = outgoing_edge_edges_start + edge_storage.vertex_len(vh(edge_storage[outgoing_offset]));
        let outgoing_edge = &mut edge_storage[outgoing_offset];

        stack.push((outgoing_edge_edges_start,
                    outgoing_edge_edges_end,
                    outgoing_edge as *mut Edge));
    }

    // Return back to the src without exploring further
    while stack.len() > 0 {
        let (ptr, end, packed_edge) = stack.pop().unwrap();
        post_order_func(unsafe { packed_edge.as_mut().unwrap() });
    }
}