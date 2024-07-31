use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use crate::handles::types::{VHandle, Weight};
use crate::traits::{StoreVertex};

pub struct FlowData {
    pub level: Weight,
    pub flow: Weight,
    pub sub_sum: Weight,
}

pub struct DinicVertexWrapper<'a, VertexType, VertexStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>
{
    vertices: &'a mut VertexStorageType,
    pub flow_data: Vec<FlowData>,
}

impl<'a, VertexType, VertexStorageType> DinicVertexWrapper<'a, VertexType, VertexStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
{
    pub fn from(vertices: &'a mut VertexStorageType) -> Self {
        let len = vertices.len();
        DinicVertexWrapper {
            vertices,
            flow_data: Vec::with_capacity(len),
        }
    }
}

impl<'a, VertexType, VertexStorageType> Index<VHandle> for DinicVertexWrapper<'a, VertexType, VertexStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
{
    type Output = VertexType;

    fn index(&self, index: VHandle) -> &Self::Output {
        self.vertices.index(index)
    }
}

impl<'a, VertexType, VertexStorageType> IndexMut<VHandle> for DinicVertexWrapper<'a, VertexType, VertexStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
{
    fn index_mut(&mut self, index: VHandle) -> &mut Self::Output {
        self.index_mut(index)
    }
}
impl<'a, VertexType, VertexStorageType> StoreVertex for DinicVertexWrapper<'a, VertexType, VertexStorageType>
where
    VertexStorageType: StoreVertex<VertexType=VertexType>,
{
    type VertexType = VertexType;
    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn push(&mut self, val: VertexType) {
        self.vertices.push(val);
        self.flow_data.push({
            FlowData {
                level: Weight::MAX,
                flow: Weight::MAX,
                sub_sum: Weight::MAX,
            }
        });
    }
    fn capacity(&self) -> usize {
        self.vertices.capacity()
    }

    fn iter(&self) -> Iter<VertexType> {
        self.vertices.iter()
    }

    fn iter_mut(&mut self) -> IterMut<VertexType> {
        self.vertices.iter_mut()
    }

    fn as_slice(&self) -> &[VertexType] {
        self.vertices.as_slice()
    }
}

pub fn mark_levels() {}

// pub fn hybrid_dinic<VertexType, EdgeStorageType>(graph: WeightedGraph<VertexType, EdgeStorageType>) -> WeightedGraph<DinicVertexStorage<VertexType>, EdgeStorageType>
// where EdgeStorageType: WeightedManipulate {
//     let mut edges = graph.graph.edges.clone();
//     let mut vertices = DinicVertexStorage::from(&graph.graph.vertices);
//     // let new_graph = Graph{
//     //     vertices,
//     //     edges,
//     // };
//
// }
