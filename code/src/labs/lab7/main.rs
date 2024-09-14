use std::time::Instant;

use crate::libs::stack::list_stack::Stack as ListStack;
use crate::libs::stack::vector_stack::Stack as VectorStack;
use crate::libs::stack::StackMethods;

pub fn perf_test_stack<T: StackMethods<i32>>(mut stack: T) -> (u128, u128) {
    let now = Instant::now();
    for _ in 0..10000 {
        stack.push(1);
    }
    let push = now.elapsed().as_nanos();

    let now = Instant::now();
    for _ in 0..10000 {
        stack.pop();
    }
    let pop = now.elapsed().as_nanos();

    return (push, pop);
}

pub fn main() {
    let time_list = perf_test_stack(ListStack::new());
    let time_vector = perf_test_stack(VectorStack::new());

    println!(
        "List Stack Times (Push/Pop): {} ns / {} ns",
        time_list.0, time_list.1
    );
    println!(
        "Vector Stack Times (Push/Pop): {} ns / {} ns",
        time_vector.0, time_vector.1
    );
}
