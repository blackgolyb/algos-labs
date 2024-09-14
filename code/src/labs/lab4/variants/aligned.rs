use std::mem::ManuallyDrop;

use crate::libs::bytes::{impl_show_bytes, show, ShowBytes};

#[repr(C)]
pub struct AlignedPc {
    pub filed1: u8,
    pub filed2: i32,
    pub filed3: u8,
}

#[repr(C)]
pub struct AlignedTable {
    pub filed1: u8,
    pub filed2: i32,
}

#[repr(C)]
pub union AlignedUnion {
    pub pc: ManuallyDrop<AlignedPc>,
    pub table: ManuallyDrop<AlignedTable>,
}

#[repr(C)]
pub struct Aligned {
    pub kind: AlignedUnion,
    pub for_child: bool,
    pub cooperative: bool,
}
impl_show_bytes!(Aligned);
