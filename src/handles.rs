use std::ops::AddAssign;
use crate::handles::types::{MASK, SHIFT, VHandle, Weight, PackedEdge, UNSET};
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
    pub type PackedEdge = u64;
    pub type Weight = i32;
    pub type VHandle = u32;
    pub(in crate::handles) const SHIFT: usize = 32;
    pub(in crate::handles) const MASK: u32 = 0xFFFFFFFF;
    pub const UNSET: u32 = MASK;
}

pub type Slot = PackedEdge;

pub const NONE: VHandle = VHandle::MAX;

#[inline(always)]
pub fn vh(handle: PackedEdge) -> VHandle {
    handle as VHandle
}

#[inline(always)]
pub fn wgt(handle: PackedEdge) -> Weight {
    (handle >> SHIFT) as Weight
}
#[inline(always)]
pub fn vh_pack(handle: VHandle) -> PackedEdge {
    handle as PackedEdge | ((UNSET as PackedEdge) << SHIFT)
}

#[inline(always)]
pub fn pack(node_id: VHandle, weight: Weight) -> PackedEdge {
    (node_id as PackedEdge) | ((weight as PackedEdge) << SHIFT)
}
#[inline(always)]
pub fn set_wgt(handle: PackedEdge, weight: Weight) -> PackedEdge {
    (handle & (MASK as PackedEdge)) | ((weight as PackedEdge) << SHIFT)
}
#[inline(always)]
pub fn set_vid(handle: PackedEdge, vert_id: VHandle) -> PackedEdge {
    (handle & !(MASK as PackedEdge)) | (vert_id as PackedEdge)
}