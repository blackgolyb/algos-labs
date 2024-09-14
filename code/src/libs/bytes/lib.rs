pub fn print_bytes(ptr: *const u8, size: usize) {
    for i in 0..size {
        let byte = unsafe { *ptr.add(i) };
        for j in 0..8 {
            print!("{}", (byte >> (7 - j)) & 1);
        }
        print!(" ");
    }
    println!();
}

pub fn show<T>(value: &T) {
    let size = size_of::<T>();
    let ptr = value as *const T as *const u8;
    print_bytes(ptr, size);
}

pub trait ShowBytes {
    fn show_bytes(&self);
}

#[macro_export]
macro_rules! impl_show_bytes {
    ($impl_type:ident) => {
        impl ShowBytes for $impl_type {
            fn show_bytes(&self) {
                show::<$impl_type>(&self);
            }
        }
    };
}
