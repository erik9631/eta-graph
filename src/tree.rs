
pub struct EdgeView<'a>{
    pub edges: &'a mut [usize],
}

impl <'a> EdgeView<'a> {
    pub fn new<T>(graph: &'a mut Graph<T>, vertex: usize) -> Self {
        let data_start = graph.indices[vertex];
        let data_end = graph.edge_count;
        return EdgeView{
            edges: &mut graph.edges[data_start.. data_start + data_end],
        }
    }

    #[inline(always)]
    pub fn size(&mut self) -> &mut usize {
        return &mut self.edges[0];
    }
    #[inline(always)]
    pub fn push(&mut self, vertex: usize){
        self.edges[*self.size() + 1] = vertex; // First element is size
        *self.size() += 1;
    }
}

pub struct Graph<T> {
    edge_count: usize,
    vertices: Vec<T>,
    edges: Vec<usize>,
    indices: Vec<usize>,
}
#[inline(always)]
fn calculate_new_edges_size<T>(graph: &Graph<T>) -> usize {
    return graph.edges.len() + graph.edge_count
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
    let mut edge_view = EdgeView::new(graph, from);
    edge_view.push(to);
}

pub fn create<T>(graph: &mut Graph<T>, val: T) -> usize {
    let new_edge_entry_size = graph.edges.len();
    graph.vertices.push(val);

    graph.edges.resize_with(calculate_new_edges_size(graph), Default::default);
    graph.edges[new_edge_entry_size] = 0;

    graph.indices.push(new_edge_entry_size); //Header of the new edge entry

    return graph.vertices.len() - 1;
}

pub fn create_and_connect<T>(graph: &mut Graph<T>, src_vertex: usize, val: T) -> usize {
    let new_vertex = create(graph, val);
    connect(graph, src_vertex, new_vertex);
    return new_vertex;
}

pub fn get<T> (graph: &Graph<T>, vertex: usize) -> &T {
    return &graph.vertices[vertex];
}

pub fn get_connections<T> (graph: &mut Graph<T>, vertex: usize) -> EdgeView {
    let edge_view = EdgeView::new(graph, vertex);
    return edge_view;
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

