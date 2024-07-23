use std::mem::size_of;
use std::ops::AddAssign;
use crate::handles::types::{MASK, SHIFT, Vid, VHandle, Weight};

#[cfg(msize_type = "u8")]
pub mod types{
    pub type VHandle = u8;
    pub type Weight = compile_error!("Weight is not supported when Weight type is u8");
}

#[cfg(msize_type = "u16")]
pub mod types{
    pub type VHandle = u16;
    pub type Weight = i8;
    pub type VertId = u8;
    pub const BIT_SIZE: usize = 8;
    pub(in crate::handles) const SHIFT: usize = 8;
    pub(in crate::handles) const MASK: u8 = 0xFF;
}


#[cfg(msize_type = "u32")]
pub mod types{
    pub type VHandle = u32;
    pub type Weight = i16;
    pub type Vid = u16;
    pub(in crate::handles) const SHIFT: usize = 16;

    pub(in crate::handles) const MASK: u16 = 0xFFFF;

}



#[cfg(msize_type = "u64")]
pub mod types {
    pub type VHandle = u64;
    pub type Weight = u32;
    pub type VertId = u32;
    pub(in crate::handles) const SHIFT: usize = 32;
    pub(in crate::handles) const MASK: u32 = 0xFFFFFFFF;
}

#[inline(always)]
pub fn vid(handle: VHandle) -> Vid {
    handle as Vid
}

#[inline(always)]
pub fn wgt(handle: VHandle) -> Weight {
    (handle >> SHIFT) as Weight
}

#[inline(always)]
pub fn pack(node_id: Vid, weight: Weight) -> VHandle {
    (node_id as VHandle) | ((weight as VHandle) << SHIFT)
}
#[inline(always)]
pub fn set_wgt(handle: VHandle, weight: Weight) -> VHandle {
    (handle & (MASK as VHandle)) | ((weight as VHandle) << SHIFT)
}
#[inline(always)]
pub fn set_vid(handle: VHandle, vert_id: Vid) -> VHandle {
    (handle & !(MASK as VHandle)) | (vert_id as VHandle)
}
pub(crate) const MSIZE_ALIGN_MASK: usize = size_of::<VHandle>() - 1;