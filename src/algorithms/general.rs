use crate::handles::types::{Edge, Weight};
use crate::handles::{eh, Slot};
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
    let mut was_queued_flags = Array::new_default_bytes(vertices_count, 0);
    let mut visit_queue = Queue::<*mut Edge>::new_pow2_sized(vertices_count);
    let mut end = 1;
    let mut next_layer = 1;
    let mut layer = 0;
    let mut start_edge = start;
    visit_queue.push((&mut start_edge) as *mut Edge);
    was_queued_flags[0] = true;
    let mut i = 0;

    while visit_queue.len() != 0 {
        let handle = visit_queue.dequeue().unwrap();
        match pre_order(unsafe { handle.as_mut().unwrap() }, layer) { // the i is a place holder for the layer
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
        let mut edge_iter = edge_storage.entry_iter_mut(eh(unsafe { *handle }));
        for edge in edge_iter {
            if unsafe { *was_queued_flags.index_unchecked(eh(*edge) as usize) } {
                continue;
            }
            unsafe { *was_queued_flags.index_unchecked_mut(eh(*edge) as usize) = true };
            visit_queue.push(edge as *mut Edge);
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
        let was_visited = flags[eh(to_visit) as usize];
        flags[eh(to_visit) as usize] = true;
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
    let mut stack = Stack::<(Slot, Slot, *mut Slot)>::new(vertex_count);
    stack.push((edge_storage.entry_index(eh(start)), edge_storage.entry_index(eh(start)) + edge_storage.entry_len(eh(start)), (&mut start_edge) as *mut Edge));
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

        let outgoing_edge_edges_start = edge_storage.entry_index(eh(edge_storage[outgoing_offset]));
        let outgoing_edge_edges_end = outgoing_edge_edges_start + edge_storage.entry_len(eh(edge_storage[outgoing_offset]));
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