use std::mem::size_of;

#[cfg(msize_type = "u8")]
pub type MSize = u8;

#[cfg(msize_type = "u16")]
pub type MSize = u16;

#[cfg(msize_type = "u32")]
pub type VHandle = u32;

#[cfg(msize_type = "usize")]
pub type MSize = usize;
pub(crate) const MSIZE_ALIGN_MASK: usize = size_of::<VHandle>() - 1;