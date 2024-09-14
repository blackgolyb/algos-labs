use std::mem::ManuallyDrop;

use crate::libs::bytes::{impl_show_bytes, show, ShowBytes};

pub struct StandardPc {
    pub filed1: u8,
    pub filed2: i32,
    pub filed3: u8,
}

pub struct StandardTable {
    pub filed1: u8,
    pub filed2: i32,
}

pub union StandardUnion {
    pub pc: ManuallyDrop<StandardPc>,
    pub table: ManuallyDrop<StandardTable>,
}

pub struct Standard {
    pub kind: StandardUnion,
    pub for_child: bool,
    pub cooperative: bool,
}
impl_show_bytes!(Standard);
