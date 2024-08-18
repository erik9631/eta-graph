use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use crate::handles::types::VHandle;
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

    #[inline(always)]
    pub fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }
}

impl<VertexType> Index<VHandle> for VertexStorage<VertexType> {
    type Output = VertexType;
    #[inline(always)]
    fn index(&self, index: VHandle) -> &Self::Output {
        return self.data.index(index as usize);
    }
}

impl<VertexType> IndexMut<VHandle> for VertexStorage<VertexType> {
    #[inline(always)]
    fn index_mut(&mut self, index: VHandle) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl<VertexType> Clone for VertexStorage<VertexType>
where VertexType: Clone {
    #[inline(always)]
    fn clone(&self) -> Self {
        VertexStorage {
            data: self.data.clone(),
        }
    }
}
impl <VertexType> StoreVertex for VertexStorage<VertexType>
{
    type VertexType = VertexType;
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }
    #[inline(always)]
    fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }
    #[inline(always)]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
    #[inline(always)]
    fn iter(&self) -> Iter<VertexType>{
        self.data.iter()
    }
    #[inline(always)]
    fn iter_mut(&mut self) -> IterMut<VertexType> {
        self.data.iter_mut()
    }

    #[inline(always)]
    fn as_slice(&self) -> &[VertexType] {
        self.data.as_slice()
    }
}