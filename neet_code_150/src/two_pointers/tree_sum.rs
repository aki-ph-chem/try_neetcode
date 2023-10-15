use std::collections::{HashMap, HashSet};
use std::cmp::Ordering::{Less,Greater,Equal};

struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set = HashSet::new();
        let mut result = vec![];

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let diff = -nums[i] - nums[j];

                if set.contains(&diff) {
                    result.push(vec![nums[i], nums[j], diff]);
                } else {
                    set.insert(nums[i]);
                }
            }
        }
        result
    }
}

struct SolutionAns {}
impl SolutionAns {
    pub fn three_sum(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
        numbers.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for i in 0..numbers.len() {
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, numbers.len() - 1);

            while l < r {
                match (numbers[i] + numbers[l] + numbers[r]).cmp(&0) {
                    Less => l += 1,
                    Greater => r -= 1,
                    Equal => {
                        ans.push(vec![numbers[i], numbers[l], numbers[r]]);
                        l += 1;
                        while numbers[l] == numbers[l - 1] && l < r {
                            l += 1;
                        }
                        r -= 1;
                        while numbers[r] == numbers[r + 1] && l < r {
                            r -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    let case_1 = vec![-1, 0, 1, 2, -1, -4];
    let case_2 = vec![0, 1, 1];
    let case_3 = vec![0, 0, 0];
    let case_4 = vec![-1, 1, 0];

    println!("case_1: {:?}", Solution::three_sum(case_1.clone()));
    println!("case_2: {:?}", Solution::three_sum(case_2.clone()));
    println!("case_3: {:?}", Solution::three_sum(case_3.clone()));
    println!("case_4: {:?}", Solution::three_sum(case_4.clone()));

    println!("case_1: {:?}", SolutionAns::three_sum(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::three_sum(case_2.clone()));
    println!("case_3: {:?}", SolutionAns::three_sum(case_3.clone()));
    println!("case_4: {:?}", SolutionAns::three_sum(case_4.clone()));
}
