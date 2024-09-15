use crate::libs::list::non_linear_list::MultiList;

fn present() {
    let mut list = match MultiList::<i64>::new(None, None) {
        Ok(list) => list,
        Err(_) => {
            panic!("Cannot create list")
        }
    };

    for i in 0..10 {
        list.insert_value(vec![i], i);
        for j in 0..=i {
            list.insert_value(vec![i, j], j);
            for k in 0..=j {
                list.insert_value(vec![i, j, k], k);
            }
        }
    }

    println!("{}", list);

    // for i in 0..10 {
    //     match list.get_value(vec![i]) {
    //         None => (),
    //         Some(value) => {
    //             println!("({}) -> {}", i, value);
    //         }
    //     };
    // }

    // println!();

    // for i in 0..10 {
    //     for j in 0..10 {
    //         match list.get_value(vec![i, j]) {
    //             None => (),
    //             Some(value) => {
    //                 println!("({}, {}) -> {}", i, j, value);
    //             }
    //         };
    //     }
    // }
}

fn lab() {
    let mut list = match MultiList::<String>::new(Some(2), None) {
        Ok(list) => list,
        Err(_) => {
            panic!("Cannot create list")
        }
    };

    for i in 0..10 {
        let station = format!("Path {i}");
        list.insert_value(vec![i], station);
        for j in 0..=i {
            let parh = format!("Staton {j}");
            list.insert_value(vec![i, j], parh);
        }
    }

    for i in 0..10 {
        let elem = list.get_value(vec![i]);
        print!("{:?} ", elem)
    }
    println!("\n");

    for i in 0..10 {
        let elem = list.get_value(vec![4, i]);
        print!("{:?} ", elem)
    }
    println!("\n");

    println!("{}", list);
    list.insert_list(vec![2, 2]);
    list.delete_value(vec![2, 1]);
    list.delete_list(vec![5]);
    println!("{}", list);
}

pub fn main() {
    // present();
    lab();
}
