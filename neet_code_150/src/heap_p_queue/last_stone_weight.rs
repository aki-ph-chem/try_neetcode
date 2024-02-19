use std::collections::BinaryHeap;

// AC
struct Solution {}
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(stones);

        while max_heap.len() > 1 {
            let y = max_heap.peek().unwrap().clone();
            max_heap.pop();
            let x = max_heap.peek().unwrap().clone();
            max_heap.pop();

            if x != y {
                max_heap.push(y - x);
            }
        }

        if max_heap.is_empty() {
            return 0;
        }
        *max_heap.peek().unwrap()
    }
}

// 模範解答
struct SolutoinAns {}
impl SolutoinAns {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut sotone_heap = BinaryHeap::new();
        for stone in stones {
            sotone_heap.push(stone);
        }

        while sotone_heap.len() > 1 {
            let first = sotone_heap.pop().unwrap();
            let second = sotone_heap.pop().unwrap();
            sotone_heap.push(first - second);
        }

        match sotone_heap.peek() {
            Some(val) => *val,
            None => 0,
        }
    }
}

fn main() {
    let case_1 = vec![2, 7, 4, 1, 8, 1];
    // => 1
    let case_2 = vec![1];
    // => 1

    println!("case_1: {}", Solution::last_stone_weight(case_1.clone()));
    println!("case_2: {}", Solution::last_stone_weight(case_2.clone()));
}
