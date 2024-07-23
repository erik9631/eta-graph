use std::alloc::{alloc, Layout};
use std::slice::from_raw_parts_mut;
use firestorm::profile_method;
use crate::graph::TraverseResult;
use crate::graph::TraverseResult::End;
use crate::handles::types::VHandle;
use crate::traits::{EdgeStore, TraverseMarker};

pub fn bfs<TraverseFunc, GraphType>(graph: &mut GraphType, start: VHandle, vertices_count: usize, mut transform: TraverseFunc) where
        TraverseFunc: FnMut(&mut GraphType, VHandle) -> TraverseResult,
        GraphType: EdgeStore + TraverseMarker{
    profile_method!(bfs);
    let layout = Layout::array::<VHandle>(vertices_count).expect("Failed to create layout"); // Around ~50% faster than vec
    let to_visit = unsafe {from_raw_parts_mut(alloc(layout) as *mut VHandle, vertices_count)};
    let mut end = 1;
    to_visit[0] = start;
    let mut i = 0;
    while i != end {
        let handle = to_visit[i];
        if transform(graph, handle) == End{
            graph.inc_global_visited_flag();
            break;
        }
        graph.inc_visited_flag(handle);

        let edges = graph.edges(handle);
        for next in edges {
            if graph.visited_flag(*next) == graph.global_visited_flag() {
                continue;
            }
            to_visit[end] = *next;
            end += 1;
        }
        i +=1;
    }
    graph.reset_global_visited_flag(); // Reset the visited flag as we traversed the whole graph
}
