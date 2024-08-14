use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use crate::handles::types::EHandle;
use crate::traits::{StoreVertex};

pub struct VertexStorage<VertexType> {
    data: Vec<VertexType>,
}
impl<VertexType> Default for VertexStorage<VertexType> {
    fn default() -> Self {
        Self::new()
    }
}

impl <VertexType> VertexStorage<VertexType> {
    pub fn new() -> Self {
        VertexStorage {
            data: Vec::new(),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<VertexType> Index<EHandle> for VertexStorage<VertexType> {
    type Output = VertexType;

    fn index(&self, index: EHandle) -> &Self::Output {
        return self.data.index(index as usize);
    }
}

impl<VertexType> IndexMut<EHandle> for VertexStorage<VertexType> {
    fn index_mut(&mut self, index: EHandle) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl<VertexType> Clone for VertexStorage<VertexType>
where VertexType: Clone {
    fn clone(&self) -> Self {
        VertexStorage {
            data: self.data.clone(),
        }
    }
}
impl <VertexType> StoreVertex for VertexStorage<VertexType>
{
    type VertexType = VertexType;
    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn iter(&self) -> Iter<VertexType>{
        return self.data.iter();
    }

    fn iter_mut(&mut self) -> IterMut<VertexType> {
        return self.data.iter_mut();
    }

    fn as_slice(&self) -> &[VertexType] {
        return self.data.as_slice();
    }
}