use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

pub fn print_raw_bytes(byte: usize, size: usize) {
    print_bytes(&byte as *const usize as *const u8, size);
}

pub fn print_byte(ptr: *const u8) {
    let byte = unsafe { *ptr };
    for j in 0..8 {
        print!("{}", (byte >> (7 - j)) & 1);
    }
}

pub fn print_bytes(ptr: *const u8, size: usize) {
    for i in 0..(size - 1) {
        print_byte(unsafe { ptr.add(i) });
        print!(" ");
    }

    if size > 0 {
        print_byte(unsafe { ptr.add(size - 1) });
    }
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

impl_show_bytes!(i8);
impl_show_bytes!(i16);
impl_show_bytes!(i32);
impl_show_bytes!(i64);
impl_show_bytes!(i128);

impl_show_bytes!(u8);
impl_show_bytes!(u16);
impl_show_bytes!(u32);
impl_show_bytes!(u64);
impl_show_bytes!(u128);

impl_show_bytes!(f32);
impl_show_bytes!(f64);

impl_show_bytes!(usize);
impl_show_bytes!(bool);

impl ShowBytes for char {
    fn show_bytes(&self) {
        let s = self.to_string();
        let bytes = s.as_bytes();
        for byte in bytes {
            print_byte(byte);
        }
        println!();
    }
}

#[derive(Debug)]
pub enum ParsedItem<T> {
    NestedVec(ParsedVec<T>),
    Value(T),
}

#[derive(Debug)]
pub struct ParsedVec<T>(Vec<ParsedItem<T>>);

impl<T> From<Vec<T>> for ParsedVec<T> {
    fn from(value: Vec<T>) -> Self {
        let mut res = ParsedVec(Vec::new());
        for v in value {
            res.0.push(ParsedItem::Value(v))
        }
        res
        // зроблено так щоб Rust не зміг тут оптимізувати копіювання вектора
        // ParsedVec(value.into_iter().map(|v| ParsedItem::Value(v)).collect())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVecError;

impl<T: FromStr + Debug> FromStr for ParsedVec<T>
where
    T::Err: std::fmt::Debug,
{
    type Err = ParseVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .and_then(|s| Some(s.trim()))
            .ok_or(ParseVecError)?;

        let mut items = Vec::new();

        // for empty list/sublist []
        if s.trim().is_empty() {
            return Ok(ParsedVec(items));
        }

        let mut is_err = false;
        for token in s.split(',') {
            let parsed = token.trim().parse::<T>();
            match parsed {
                Err(_) => {
                    is_err = true;
                    break;
                }
                Ok(val) => items.push(ParsedItem::Value(val)),
            }
        }

        if !is_err {
            return Ok(ParsedVec(items));
        }

        let mut items = Vec::new();
        let mut depth = 0;
        let mut start = 0;
        let mut i = 0;
        let mut is_comma = true;
        let mut is_value = false;

        while i < s.len() {
            match &s[i..=i] {
                "[" => {
                    if depth == 0 {
                        if !is_comma {
                            return Err(ParseVecError);
                        }
                        is_comma = false;
                        start = i;
                    }
                    depth += 1;
                }
                "]" => {
                    if depth == 1 {
                        let parsed = s[start..=i].parse::<ParsedVec<T>>()?;
                        items.push(ParsedItem::NestedVec(parsed));
                    }
                    depth -= 1;
                }
                "," if depth == 0 => {
                    if is_comma {
                        return Err(ParseVecError);
                    }
                    if is_value {
                        let parsed = s[start..i].trim().parse::<T>();
                        if parsed.is_err() {
                            return Err(ParseVecError);
                        }
                        is_value = false;
                        items.push(ParsedItem::Value(parsed.unwrap()));
                    }
                    is_comma = true;
                }
                " " => (),
                "\n" => (),
                "\t" => (),
                _ if depth == 0 && is_comma && !is_value => {
                    is_value = true;
                    is_comma = false;
                    start = i;
                }
                _ => (),
            }
            i += 1;
        }

        if is_value {
            let parsed = s[start..].trim().parse::<T>();
            if parsed.is_err() {
                return Err(ParseVecError);
            }
            items.push(ParsedItem::Value(parsed.unwrap()));
        }

        Ok(ParsedVec(items))
    }
}

impl<T: Display> Display for ParsedItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedItem::Value(val) => write!(f, "{}", val),
            ParsedItem::NestedVec(vec) => write!(f, "{}", vec),
            _ => Ok(()),
        }
    }
}

impl<T: Display> Display for ParsedVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: String = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", content)
    }
}

fn print_layers(layers: &Vec<bool>) {
    let s: String = layers
        .iter()
        .map(|l| match l {
            true => "│   ",
            false => "    ",
        })
        .collect();
    print!("{}", s);
}

fn show_bytes_vec<T: Display + ShowBytes>(vec: &Vec<T>, layers: &Vec<bool>) {
    print_layers(layers);
    println!("│{{");
    print_layers(layers);
    print!("│    addr: {:p} (", vec);
    let addr = vec as *const Vec<T> as usize;
    print_raw_bytes(addr, 8);
    println!(")");

    print_layers(layers);
    print!("│    capacity: {} (\x1b[34m", vec.capacity());
    vec.capacity().show_bytes();
    println!("\x1b[0m)");

    print_layers(layers);
    print!("│    length: {} (\x1b[32m", vec.len());
    vec.len().show_bytes();
    println!("\x1b[0m)");

    print_layers(layers);
    print!("│    ptr: \x1b[31m");
    (vec.as_ptr() as usize).show_bytes();
    println!("\x1b[0m");

    // let ptr = vec as *const Vec<T> as *const u8;
    // print_layers(layers);
    // print!("│    bin: \x1b[34m");
    // print_bytes(ptr, 8);
    // print!(" \x1b[31m");
    // print_bytes(unsafe { ptr.add(8) }, 8);
    // print!(" \x1b[32m");
    // print_bytes(unsafe { ptr.add(16) }, 8);
    // println!("\x1b[0m");

    // if vec.len() == 0 {
    //     return;
    // }

    // let elem_size = size_of::<T>();
    // let size = elem_size * vec.capacity();
    // let actual_size = elem_size * vec.len();
    // let ptr = vec.as_ptr() as *const u8;

    // print!("Inner: ");
    // for i in 0..size {
    //     if i >= actual_size {
    //         print!("\x1b[90m");
    //     }
    //     if i % elem_size == 0 && i != 0 {
    //         print!("| ");
    //     }
    //     print_byte(unsafe { ptr.add(i) });
    //     print!(" ");
    // }
    // println!("\x1b[0m");

    print_layers(layers);
    println!("│}}");
}

impl<T: ShowBytes + Display> ParsedItem<T> {
    pub fn show_bytes_(&self, layers: &mut Vec<bool>) {
        match self {
            ParsedItem::Value(val) => {
                println!("Value:");
                print_layers(layers);
                println!("│{{");
                print_layers(layers);
                print!("│    addr: {:p} (", val);
                let addr = val as *const T as usize;
                print_raw_bytes(addr, 8);
                println!(")");

                print_layers(layers);
                print!("│    bin: ");
                let ptr = val as *const T as usize as *const u8;
                let size = size_of::<ParsedItem<T>>();
                print_bytes(ptr, size);
                println!();

                print_layers(layers);
                println!("│    value: {val}");

                print_layers(layers);
                println!("│}}");
            }
            ParsedItem::NestedVec(vec) => {
                vec.show_bytes_(layers);
            }
        }
    }
}

impl<T: ShowBytes + Display> ShowBytes for ParsedItem<T> {
    fn show_bytes(&self) {
        self.show_bytes_(&mut Vec::new());
    }
}

impl<T: ShowBytes + Display> ParsedVec<T> {
    fn propagate_show_tree(&self, layers: &mut Vec<bool>) {
        let len = self.0.len();
        let mut inner_layers = layers.clone();
        inner_layers.push(true);

        for i in 0..(len - 1) {
            print_layers(&inner_layers);
            println!();
            print_layers(layers);
            print!("├───");
            self.0[i].show_bytes_(&mut inner_layers);
        }

        let level = inner_layers.len();
        print_layers(&inner_layers);
        inner_layers[level - 1] = false;
        println!();
        print_layers(layers);
        print!("└───");
        self.0[len - 1].show_bytes_(&mut inner_layers);
    }
    pub fn show_bytes_(&self, layers: &mut Vec<bool>) {
        if layers.is_empty() {
            println!("Vec");
        } else {
            println!("NestedVec");
        }

        show_bytes_vec(&self.0, layers);

        if self.0.is_empty() {
            return;
        }

        self.propagate_show_tree(layers);
    }
}

impl<T: ShowBytes + Display> ShowBytes for ParsedVec<T> {
    fn show_bytes(&self) {
        self.show_bytes_(&mut Vec::new());
    }
}
