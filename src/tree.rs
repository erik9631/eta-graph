use std::mem::size_of;
#[inline(always)]
fn calculate_new_edges_size<T>(graph: &Graph<T>) -> usize {
    return graph.edges.len() + graph.edge_count
}

pub trait TreeBehavior<T>{
    fn new(value: T) -> Box<Self>;
    fn new_with_capacity(value: T, capacity: usize) -> Box<Self>;
    fn new_node(&mut self, value: T) -> *mut Self;
    fn new_node_with_capacity(&mut self, value: T, capacity: usize) -> *mut Self;
    fn add_node(&mut self, node: *mut Self);
    fn remove_node(&mut self, node: *mut Self) -> Option<*mut Self>;
    fn get_root(&self) -> &Self;
    fn get_parent(&self) -> Option<&Self>;
    fn get_node(&self, index: u32) -> &Self;
    fn get_value(&self) -> &T;
    fn len(&self) -> usize;
}

struct Edge<'a>{
    size: usize,
    edges: &'a [usize],
}

pub struct Graph<T> {
    edge_count: usize,
    vertices: Vec<T>,
    edges: Vec<usize>,
    indices: Vec<usize>,
}

impl<T> Graph<T>{
    pub fn new() -> Self {
        return Graph{
            edge_count: 10 + 1, //10 edges per vertex + 1 for the header
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

pub fn connect<T>(graph: &mut Graph<T>, from: usize, to: usize) {
    let index_start = graph.indices[from];
    let from_size = &mut graph.edges[index_start];
    if *from_size > graph.edge_count {
        panic!("Edge count exceeded")
    }

    *from_size += 1;
    let from_index_new = *from_size;
    graph.edges[from_index_new] = to;
}


pub fn create<T>(graph: &mut Graph<T>, val: T) -> usize {
    let new_edge_entry_size = graph.edges.len();
    graph.vertices.push(val);

    graph.edges.resize_with(calculate_new_edges_size(graph), Default::default);
    graph.edges[new_edge_entry_size] = 0;

    graph.indices.push(new_edge_entry_size); //Header of the new edge entry

    return graph.vertices.len() - 1;
}
pub fn create_out<T>(graph: &mut Graph<T>, src_vertex: usize, val: T) -> usize {
    let new_edge_entry_size = graph.edges.len();
    let new_edge_entry = new_edge_entry_size + 1;
    graph.vertices.push(val);

    graph.edges.resize_with(calculate_new_edges_size(graph), Default::default);
    graph.edges[new_edge_entry] = src_vertex;
    graph.edges[new_edge_entry_size] += 1; //Size of 1;

    graph.indices.push(new_edge_entry_size); //Header of the new edge entry

    return graph.vertices.len() - 1;
}


pub fn get<T> (graph: &Graph<T>, vertex: usize) -> &T {
    return &graph.vertices[vertex];
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

