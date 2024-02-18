use std::cmp::Reverse;
use std::collections::BinaryHeap;

// C++のstd::priority_queue<T>に相当する
fn main() {
    // 基本的な使い方
    // デフォルトでは降順
    let mut q_max: BinaryHeap<i32> = BinaryHeap::new();
    q_max.push(5);
    q_max.push(2);
    q_max.push(3);

    println!("q_max");
    while !q_max.is_empty() {
        println!("{}", q_max.peek().unwrap());
        q_max.pop();
    }

    // 昇順で使う
    let mut q_min: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    q_min.push(Reverse(5));
    q_min.push(Reverse(2));
    q_min.push(Reverse(3));

    println!("q_min");
    while !q_min.is_empty() {
        println!("{}", q_min.peek().unwrap().0);
        q_min.pop();
    }

    // (i32, i32)を成分とする場合
    // 昇順に配置する
    let mut q_tuple: BinaryHeap<(Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
    q_tuple.push((Reverse(5), Reverse(5)));
    q_tuple.push((Reverse(2), Reverse(21)));
    q_tuple.push((Reverse(3), Reverse(1)));

    println!("q_tuple");
    while !q_tuple.is_empty() {
        println!(
            "{}, {}",
            q_tuple.peek().unwrap().0 .0,
            q_tuple.peek().unwrap().1 .0
        );
        q_tuple.pop();
    }
}
