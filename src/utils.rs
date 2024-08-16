use crate::edge_storage::EdgeStorage;
use crate::graph::Graph;
use crate::handles::types::VHandle;
use crate::handles::{vh, wgt};
use crate::traits::{EdgeStore, StoreVertex};

pub fn print_graph<VertexType, VertexStorageType, EdgeStorageType>(vertices: &VertexStorageType, edges: &EdgeStorageType)
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
    EdgeStorageType: EdgeStore,
    VertexType: std::fmt::Debug + std::fmt::Display,
{
    for (vertex, val) in vertices.iter().enumerate(){
        for edge in edges.edges_iter(vertex as VHandle){
            println!("{} --{}-> {}", *val ,wgt(*edge), vertices[vh(*edge)]);
        }
        println!("---");
    }
}