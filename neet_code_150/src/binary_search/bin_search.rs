struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            }

            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

// 模範解答
use std::cmp::Ordering::*;
struct SolutionAns {}
impl SolutionAns {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());

        while l < r {
            let m = l + (r - l) / 2;
            match target.cmp(&nums[m]) {
                Equal => return m as i32,
                Less => r = m,
                Greater => l = m + 1,
            }
        }

        -1
    }
}

fn main() {
    let case_1 = (vec![-1, 0, 3, 5, 9, 12], 9);
    let case_2 = (vec![-1, 0, 3, 5, 9, 12], 2);
    let case_3 = (vec![2, 5], 0);

    println!("case_1: {}", Solution::search(case_1.0.clone(), case_1.1));
    println!("case_2: {}", Solution::search(case_2.0.clone(), case_2.1));
    println!("case_3: {}", Solution::search(case_3.0.clone(), case_3.1));

    println!("case_1: {}", SolutionAns::search(case_1.0.clone(), case_1.1));
    println!("case_2: {}", SolutionAns::search(case_2.0.clone(), case_2.1));
    println!("case_3: {}", SolutionAns::search(case_3.0.clone(), case_3.1));
}
