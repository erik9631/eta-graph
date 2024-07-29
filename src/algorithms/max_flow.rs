
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use crate::handles::types::{VHandle, Weight};
use crate::traits::{StoreVertex, WeightedManipulate};
use crate::vertex_storage::VertexStorage;
use crate::weighted_graph::WeightedGraph;

pub struct FlowData {
    pub level: Weight,
    pub flow: Weight,
    pub sub_sum: Weight,
}

pub struct DinicVertexStorage<'a, VertexType> {
    vertices: &'a VertexStorage<VertexType>,
    pub flow_data: Vec<FlowData>,
}

impl<VertexType> DinicVertexStorage<VertexType>
where
    VertexType: Clone,
{
    pub fn from<StoreVertex, StoreVertexType>(vertices: &StoreVertex<StoreVertexType>) -> Self {
        DinicVertexStorage {
            vertices,
            flow_data: Vec::with_capacity(vertices.len()),
        }
    }
}

impl<VertexType> Index<VHandle> for DinicVertexStorage<VertexType>
where
    VertexType: Clone,
{
    type Output = (VertexType, FlowData);

    fn index(&self, index: VHandle) -> &Self::Output {
        self.vertices.index(index)
    }
}

impl<VertexType> IndexMut<VHandle> for DinicVertexStorage<VertexType>
where
    VertexType: Clone,
{
    fn index_mut(&mut self, index: VHandle) -> &mut Self::Output {
        self.vertices.index_mut(index)
    }
}

impl<VertexType, StoreVertexType> Clone for DinicVertexStorage<VertexType>
where
    VertexType: Clone,
{
    fn clone(&self) -> Self {
        DinicVertexStorage {
            vertices: self.vertices,
            flow_data: self.flow_data.clone(),
        }
    }
}

impl<VertexType, StoreVertexType> StoreVertex<VertexType> for DinicVertexStorage<VertexType>
where
    VertexType: Clone,
{
    type Item = VertexType;
    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn push(&mut self, val: Self::Item) {
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

    fn iter(&self) -> Iter<Self::Item> {
        self.vertices.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Self::Item> {
        self.vertices.iter_mut()
    }

    fn as_slice(&self) -> &[VertexType] {
        self.vertices.as_slice()
    }
}

pub fn hybrid_dinic<VertexType, EdgeStorageType>(graph: WeightedGraph<VertexType, EdgeStorageType>) -> WeightedGraph<DinicVertexStorage<VertexType>, EdgeStorageType>
where EdgeStorageType: WeightedManipulate {
    let mut edges = graph.graph.edges.clone();
    let mut vertices = DinicVertexStorage::from(&graph.graph.vertices);
}
