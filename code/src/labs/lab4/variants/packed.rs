use std::mem::ManuallyDrop;

use crate::libs::bytes::{impl_show_bytes, show, ShowBytes};

#[repr(C, packed(1))]
pub struct PackedPc {
    pub filed1: u8,
    pub filed2: i32,
    pub filed3: u8,
}

#[repr(C, packed(1))]
pub struct PackedTable {
    pub filed1: u8,
    pub filed2: i32,
}

#[repr(C, packed(1))]
pub union PackedUnion {
    pub pc: ManuallyDrop<PackedPc>,
    pub table: ManuallyDrop<PackedTable>,
}

#[repr(C, packed(1))]
pub struct Packed {
    pub kind: PackedUnion,
    pub for_child: bool,
    pub cooperative: bool,
}
impl_show_bytes!(Packed);
