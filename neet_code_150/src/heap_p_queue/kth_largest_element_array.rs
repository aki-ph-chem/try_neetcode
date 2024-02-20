use std::cmp::Reverse;
use std::collections::BinaryHeap;

// AC
// k番目に大きな要素(ただしsort()は禁止)
struct Solution {}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap_max: BinaryHeap<i32> = BinaryHeap::from(nums);

        for _step in 0..(k - 1) {
            heap_max.pop();
        }

        *heap_max.peek().unwrap()
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();

        for &n in nums.iter() {
            if heap.len() < k as usize {
                heap.push(-n);
                continue;
            } else if -heap.peek().unwrap() < n {
                heap.pop();
                heap.push(-n);
            }
        }

        -heap.pop().unwrap()
    }

    // AC
    pub fn find_kth_largest_rev(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for &n in nums.iter() {
            if heap.len() < k as usize {
                heap.push(Reverse(n));
                continue;
            } else if heap.peek().unwrap().0 < n {
                heap.pop();
                heap.push(Reverse(n));
            }
        }

        heap.pop().unwrap().0
    }
}

fn main() {
    let case_1 = (vec![3, 2, 1, 5, 6, 4], 2);
    // => 5
    let case_2 = (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
    // => 4

    println!(
        "case_1: {}",
        Solution::find_kth_largest(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::find_kth_largest(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::find_kth_largest(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::find_kth_largest(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::find_kth_largest_rev(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::find_kth_largest_rev(case_2.0.clone(), case_2.1)
    );
}
