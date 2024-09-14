use crate::libs::list::non_linear_list::MultiList;

fn present() {
    let dimentions = 3;
    let limits: [usize; 3] = [5, 5, 10];
    let mut indices: [usize; 2] = [0, 0];

    let mut list = MultiList::<i32>::new(dimentions, &limits);

    for i in 0..5 {
        indices[0] = i;
        for j in 0..=i {
            indices[1] = j;
            for k in 0..=j {
                list.push(k as i32, &indices);
            }
        }
    }

    println!("{}", list);
}

fn lab() {
    let limits: [usize; 2] = [5, 10];
    let mut indices: [usize; 1] = [0];
    let mut list = MultiList::<String>::new(2, &limits);
    let mut paths: Vec<String> = vec![];
    for i in 0..limits[0] {
        indices[1] = i;
        let station = format!("Staton {i}");
        paths.push(station);
        for j in 0..=i {
            indices[0] = j;
            let station = format!("Staton {j}");
            list.push(station, &indices);
        }
    }

    println!("{:?}", paths);
    println!("{}", list);
}

pub fn main() {
    present();
    lab();
}
