use std::mem;
use std::mem::{size_of, transmute};
type Header = usize;



// TODO use this
pub struct EdgeData{
    pub edges: Vec<usize>,
    pub indices: Vec<usize>,
}
pub struct Graph<T> {
    edge_capacity: usize,
    pub vertices: Vec<T>,
    pub edges: Vec<usize>,
    pub indices: Vec<usize>,
}
pub enum Error{
    NoHandle,
}

#[cfg_attr(release, inline(always))]
fn header_element_offset() -> usize {
    return size_of::<Header>() / size_of::<usize>();
}

#[cfg_attr(release, inline(always))]
fn calculate_new_edges_size<T>(graph: &Graph<T>) -> usize {
    return graph.edges.len() + graph.edge_capacity + (header_element_offset());
}
impl<T> Graph<T>{
    pub fn new() -> Self {
        return Graph{
            edge_capacity: 10, //10 edges per vertex + 1 for the header
            vertices: Vec::new(),
            edges: Vec::with_capacity(0),
            indices: Vec::new(),
        }
    }
}

pub fn edges_len<T>(graph: &Graph<T>, vertex: usize) -> usize {
    return graph.edges[graph.indices[vertex]];
}
pub fn edges_capacity<T>(graph: &Graph<T>) -> usize {
    return graph.edges.len();
}
#[cfg_attr(release, inline(always))]
pub fn connect<T>(graph: &mut Graph<T>, from: usize, to: usize) {
    add_edges(graph, from, &[to]);
}

pub fn create<T>(graph: &mut Graph<T>, val: T) -> usize {
    let new_data_index = graph.edges.len();
    graph.vertices.push(val);

    graph.edges.resize_with(calculate_new_edges_size(graph), Default::default);
    graph.edges[new_data_index] = 0;

    graph.indices.push(new_data_index); //Header of the new edge entry

    return graph.vertices.len() - 1;
}

pub fn create_and_connect<T>(graph: &mut Graph<T>, src_vertex: usize, val: T) -> usize {
    let new_vertex = create(graph, val);
    connect(graph, src_vertex, new_vertex);
    return new_vertex;
}
#[cfg_attr(release, inline(always))]
pub fn value<T> (vertices: &Vec<T>, vertex: usize) -> &T {
    return &vertices[vertex];
}
#[cfg_attr(release, inline(always))]
pub fn value_mut<T> (vertices: &Vec<T>, vertex: usize) -> &mut T {
    return &mut vertices[vertex];
}


pub fn edges<'a>(indices: &Vec<usize>, edges: &'a Vec<usize>, vertex: usize, offset: usize) -> Result<&'a [usize], Error> {
    let edge = indices[vertex];
    let size = edges[edge];
    if offset >= size {
        return Err(Error::NoHandle);
    }

    return Ok(&edges[edge + header_element_offset() + offset..edge + size + header_element_offset() ]);
}

pub fn add_edges<T>(graph: &mut Graph<T>, vertex: usize, edge: &[usize]) {
    let data_offset = graph.indices[vertex];
    unsafe{
        let head_ptr = graph.edges.as_mut_ptr().offset(data_offset as isize);

        let new_data_ptr = head_ptr.offset((*head_ptr + header_element_offset()) as isize);
        if new_data_ptr > new_data_ptr.offset(edge.len() as isize){
            panic!("Edge size is greater than the allocated size");
        }
        let src_data_ptr = edge.as_ptr();
        std::ptr::copy(src_data_ptr, new_data_ptr, edge.len());
        *head_ptr += edge.len();
    }
}

// pub fn bfs<T>(root: *mut Tree<T>, traverse: fn(node: &Tree<T>)){
//     let mut nodes: LinkedList<*mut Tree<T>> = LinkedList::new();
//     nodes.push_back(root);
//
//     while !nodes.is_empty() {
//         let node = nodes.front();
//         if node.is_none() {
//             return;
//         }
//         let node = node.unwrap();
//         traverse(node);
//
//         for child in node.get_children().iter() {
//             nodes.push_back(*child);
//         }
//     }
// }

// pub fn dfs<T>(root: &Tree<T>, traverse: fn(node: &Tree<T>)){
//     let mut stack: Vec<(&Tree<T>, Iter<*mut Tree<T>>)> = Vec::new();
//     stack.push( (root, root.children.iter()));
//
//     while !stack.is_empty() {
//         let current_node = stack.last_mut().unwrap();
//
//         let child_node = current_node.1.next();
//         let parent_node = current_node.0;
//         match child_node {
//             None => {
//                 stack.pop();
//                 traverse(parent_node);
//             },
//             Some(child_node) => {
//                 stack.push( (child_node, child_node.children.iter()) );
//             }
//         }
//     }
// }

