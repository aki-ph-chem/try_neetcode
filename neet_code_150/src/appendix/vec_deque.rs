use std::collections::VecDeque;

fn main() {
    // first in / first out: queueとして振る舞う
    let mut q_1: VecDeque<i32> = VecDeque::new();
    q_1.push_back(5);
    q_1.push_back(3);
    q_1.push_back(6);
    while !q_1.is_empty() {
        println!("{}", q_1.front().unwrap());
        q_1.pop_front();
    }

    // first in / last out: stackとして振る舞う
    let mut q_2: VecDeque<i32> = VecDeque::new();
    q_2.push_back(5);
    q_2.push_back(3);
    q_2.push_back(6);
    while !q_2.is_empty() {
        println!("{}", q_2.back().unwrap());
        q_2.pop_back();
    }
}
