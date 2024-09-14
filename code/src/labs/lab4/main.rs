use std::mem::ManuallyDrop;
use std::time::Instant;

use super::variants::aligned::{Aligned, AlignedPc, AlignedTable, AlignedUnion};
use super::variants::packed::{Packed, PackedPc, PackedTable, PackedUnion};
use super::variants::sandard::{Standard, StandardPc, StandardTable, StandardUnion};
use crate::libs::bytes::ShowBytes;

macro_rules! create_pc {
    ($st:ident, $u:ident, $pc:ident) => {
        $st {
            for_child: false,
            cooperative: true,
            kind: $u {
                pc: ManuallyDrop::new($pc {
                    filed1: 1,
                    filed2: 2,
                    filed3: 3,
                }),
            },
        }
    };
}

macro_rules! create_table {
    ($st:ident, $u:ident, $table:ident) => {
        $st {
            for_child: false,
            cooperative: true,
            kind: $u {
                table: ManuallyDrop::new($table {
                    filed1: 1,
                    filed2: 2,
                }),
            },
        }
    };
}

macro_rules! display {
    ($t:ident, $pc:ident, $table:ident) => {
        println!("{:=^30}", stringify!($t));
        println!("size of {}", size_of::<$t>());
        print!("Bytes representation: ");
        $pc.show_bytes();
        print!("Bytes representation: ");
        $table.show_bytes();
        println!();

        let n = 100000;

        let now = Instant::now();
        for _ in 0..n {
            unsafe {
                $pc.kind.pc.filed3;
            }
            $pc.for_child;
            $pc.cooperative;
        }
        println!("PC time: {}", now.elapsed().as_nanos().to_string());

        let now = Instant::now();
        for _ in 0..n {
            unsafe {
                $table.kind.table.filed2;
            }
            $table.for_child;
            $table.cooperative;
        }
        println!("Table time: {} ns", now.elapsed().as_nanos().to_string());
        println!("{:=^30}\n", "");
    };
}

pub fn main() {
    let packed_pc = create_pc!(Packed, PackedUnion, PackedPc);
    let packed_table = create_table!(Packed, PackedUnion, PackedTable);

    let aligned_pc = create_pc!(Aligned, AlignedUnion, AlignedPc);
    let aligned_table = create_table!(Aligned, AlignedUnion, AlignedTable);

    let sandard_pc = create_pc!(Standard, StandardUnion, StandardPc);
    let sandard_table = create_table!(Standard, StandardUnion, StandardTable);

    display!(Packed, packed_pc, packed_table);
    display!(Aligned, aligned_pc, aligned_table);
    display!(Standard, sandard_pc, sandard_table);
}
