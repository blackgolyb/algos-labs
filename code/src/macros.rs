macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(not(feature = "disable-stdout"))]
        std::println!($($rest)*)
    }
}

macro_rules! print {
    ($($rest:tt)*) => {
        #[cfg(not(feature = "disable-stdout"))]
        std::print!($($rest)*)
    }
}
