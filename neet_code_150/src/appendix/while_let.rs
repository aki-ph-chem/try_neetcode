use std::collections::VecDeque;

fn main() {
    // ベクタ
    let array = vec![5, 6, 3];
    let mut array_iter = array.iter();

    while let Some(v) = array_iter.next() {
        println!("{v}");
    }

    // queue
    let mut q_1 = VecDeque::new();
    q_1.push_back(5);
    q_1.push_back(3);
    q_1.push_back(10);
    while let Some(v) = q_1.front() {
        println!("{v}");
        q_1.pop_front();
    }
    let mut q_2 = VecDeque::new();
    q_2.push_back(5);
    q_2.push_back(3);
    q_2.push_back(10);
    while let Some(v) = q_2.pop_front() {
        println!("{v}");
    }

    // stack
    let mut stack_1 = VecDeque::new();
    stack_1.push_back(5);
    stack_1.push_back(3);
    stack_1.push_back(10);
    while let Some(v) = stack_1.pop_back() {
        println!("{v}");
    }
}
