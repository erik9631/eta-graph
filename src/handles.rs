use crate::handles::types::{MASK, SHIFT, VHandle, Weight, Edge};
#[cfg(msize_type = "u16")]
pub mod types{
    pub type PackedEdge = u16;
    pub type Weight = i8;
    pub type VHandle = u8;
    pub type Ci = u8;
    pub(in crate::handles) const SHIFT: usize = 8;
    pub(in crate::handles) const MASK: u8 = 0xFF;
    pub const UNSET: u8 = MASK;
}


#[cfg(msize_type = "u32")]
pub mod types{
    pub type PackedEdge = u32;
    pub type Weight = i16;
    pub type VHandle = u16;
    pub type Ci = u16;
    pub(in crate::handles) const SHIFT: usize = 16;
    pub(in crate::handles) const MASK: u16 = 0xFFFF;
    pub const UNSET: u16 = MASK;

}



#[cfg(msize_type = "u64")]
pub mod types {
    pub type Edge = u64;
    pub type Weight = i32;
    pub type VHandle = u32;
    pub type Ci = u32; /// Compact integer
    pub(in crate::handles) const SHIFT: usize = 32;
    pub(in crate::handles) const MASK: u32 = 0xFFFFFFFF;
}

pub const NONE: VHandle = VHandle::MAX;

///Casts to usize for convenient indexing
#[inline(always)]
pub fn vhu(handle: Edge) -> usize {
    (handle as VHandle) as usize
}
#[inline(always)]
pub fn vh(handle: Edge) -> VHandle {
    handle as VHandle
}

#[inline(always)]
pub fn wgt(handle: Edge) -> Weight {
    (handle >> SHIFT) as Weight
}
#[inline(always)]
pub fn vh_pack(handle: VHandle) -> Edge {
    handle as Edge
}
#[inline(always)]
pub fn pack(node_id: VHandle, weight: Weight) -> Edge {
    (node_id as Edge) | ((weight as Edge) << SHIFT)
}
#[inline(always)]
pub fn set_wgt(handle: Edge, weight: Weight) -> Edge {
    (handle & !((MASK as Edge) << SHIFT)) | ((weight as Edge) << SHIFT)
}
#[inline(always)]
pub fn set_vh(handle: Edge, vert_id: VHandle) -> Edge {
    (handle & !(MASK as Edge)) | (vert_id as Edge)
}