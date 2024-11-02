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
                    filed1: 12,
                    filed2: !0,
                    filed3: 26,
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
                    filed1: 12,
                    filed2: !0,
                }),
            },
        }
    };
}

macro_rules! display {
    ($t:ident, $pc:ident, $table:ident) => {
        println!("{:=^30}", stringify!($t));
        println!("size of {}", size_of::<$t>());
        print!("PC bytes representation: ");
        $pc.show_bytes();
        println!();
        print!("    for_child: ");
        $pc.for_child.show_bytes();
        println!();
        print!("    cooperative: ");
        $pc.cooperative.show_bytes();
        println!();
        unsafe {
            let f1 = $pc.kind.pc.filed1;
            let f2 = $pc.kind.pc.filed2;
            let f3 = $pc.kind.pc.filed3;
            print!("    filed1: ");
            f1.show_bytes();
            println!();
            print!("    pc filed2: ");
            f2.show_bytes();
            println!();
            print!("    pc filed3: ");
            f3.show_bytes();
            println!();
        }

        println!();

        print!("Table bytes representation: ");
        $table.show_bytes();
        println!();
        print!("    for_child: ");
        $pc.for_child.show_bytes();
        println!();
        print!("    cooperative: ");
        $pc.cooperative.show_bytes();
        println!();
        unsafe {
            let f1 = $pc.kind.table.filed1;
            let f2 = $pc.kind.table.filed2;
            print!("    filed1: ");
            f1.show_bytes();
            println!();
            print!("    table filed2: ");
            f2.show_bytes();
            println!();
        }

        let n = 10000000;

        let now = Instant::now();
        for _ in 0..n {
            unsafe {
                $pc.kind.pc.filed3;
            }
            $pc.for_child;
            $pc.cooperative;
        }
        println!("PC time: {} ns  repeats: {n}", now.elapsed().as_nanos().to_string());

        let now = Instant::now();
        for _ in 0..n {
            unsafe {
                $table.kind.table.filed2;
            }
            $table.for_child;
            $table.cooperative;
        }
        println!("Table time: {} ns  repeats: {n}", now.elapsed().as_nanos().to_string());
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
