use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use crate::handles::types::VHandle;
use crate::traits::{StoreVertex};

pub struct VertexStorage<VertexType> {
    data: Vec<VertexType>,
}
impl <VertexType> VertexStorage<VertexType> {
    pub fn new() -> Self {
        return VertexStorage {
            data: Vec::new(),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }
    pub fn len(&self) -> usize {
        return self.data.len();
    }
}
// impl <VertexType> traits::AsyncTransform for VertexStorage<VertexType>
// where VertexType: Send {
//     type ItemType = VertexType;
//
//     fn async_transform(&mut self, transform_fn: fn(&mut [VertexType])) {
//         let max_parallelism = available_parallelism().ok().unwrap().get();
//         let parallelism_count = min(max_parallelism, self.data.len());
//         let parts = split_to_parts_mut(&mut self.data, parallelism_count);
//
//         std::thread::scope(|scope| {
//             for part in parts {
//                 scope.spawn(|| {
//                     transform_fn(part);
//                 });
//             }
//         });
//     }
// }

impl<VertexType> Index<VHandle> for VertexStorage<VertexType> {
    type Output = VertexType;

    fn index(&self, index: VHandle) -> &Self::Output {
        return self.data.index(index as usize);
    }
}

impl<VertexType> IndexMut<VHandle> for VertexStorage<VertexType> {
    fn index_mut(&mut self, index: VHandle) -> &mut Self::Output {
        return &mut self.data[index as usize];
    }
}

impl<VertexType> Clone for VertexStorage<VertexType>
where VertexType: Clone {
    fn clone(&self) -> Self {
        return VertexStorage {
            data: self.data.clone(),
        };
    }
}
impl <VertexType> StoreVertex for VertexStorage<VertexType>
{
    type VertexType = VertexType;
    fn len(&self) -> usize {
        return self.data.len();
    }

    fn push(&mut self, val: VertexType) {
        self.data.push(val);
    }

    fn capacity(&self) -> usize {
        return self.data.capacity();
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