use crate::handles::types::{MASK, SHIFT, EHandle, Weight, Edge};
#[cfg(msize_type = "u16")]
pub mod types{
    pub type PackedEdge = u16;
    pub type Weight = i8;
    pub type VHandle = u8;
    pub(in crate::handles) const SHIFT: usize = 8;
    pub(in crate::handles) const MASK: u8 = 0xFF;
    pub const UNSET: u8 = MASK;
}


#[cfg(msize_type = "u32")]
pub mod types{
    pub type PackedEdge = u32;
    pub type Weight = i16;
    pub type VHandle = u16;
    pub(in crate::handles) const SHIFT: usize = 16;
    pub(in crate::handles) const MASK: u16 = 0xFFFF;
    pub const UNSET: u16 = MASK;

}



#[cfg(msize_type = "u64")]
pub mod types {
    pub type Edge = u64;
    pub type Weight = i32;
    pub type EHandle = u32;
    pub(in crate::handles) const SHIFT: usize = 32;
    pub(in crate::handles) const MASK: u32 = 0xFFFFFFFF;
}

pub type Slot = Edge;

pub const NONE: EHandle = EHandle::MAX;

#[inline(always)]
pub fn eh(handle: Edge) -> EHandle {
    handle as EHandle
}

#[inline(always)]
pub fn wgt(handle: Edge) -> Weight {
    (handle >> SHIFT) as Weight
}
#[inline(always)]
pub fn eh_pack(handle: EHandle) -> Edge {
    handle as Edge
}
#[inline(always)]
pub fn pack(node_id: EHandle, weight: Weight) -> Edge {
    (node_id as Edge) | ((weight as Edge) << SHIFT)
}
#[inline(always)]
pub fn set_wgt(handle: Edge, weight: Weight) -> Edge {
    (handle & !((MASK as Edge) << SHIFT)) | ((weight as Edge) << SHIFT)
}
#[inline(always)]
pub fn set_eh(handle: Edge, vert_id: EHandle) -> Edge {
    (handle & !(MASK as Edge)) | (vert_id as Edge)
}