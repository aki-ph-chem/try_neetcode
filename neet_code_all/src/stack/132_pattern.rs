use std::collections::VecDeque;

// 解けなかった😹
struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut de_q = VecDeque::new();
        for n in nums {
            if de_q.len() < 2 {
                de_q.push_back(n);
            } else {
                let num_i = *de_q.front().unwrap();
                let num_j = *de_q.back().unwrap();
                let num_k = n;

                if num_i < num_k && num_k < num_j {
                    return true;
                } else {
                    de_q.pop_front();
                    de_q.push_back(n);
                }
            }
            println!("n, de_q: {},{:?}", n, de_q);
        }

        false
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<(i32, i32)> = vec![];
        let mut current_min = nums[0];

        for n in nums.iter().skip(1) {
            while !stack.is_empty() && *n >= stack.last().unwrap().0 {
                stack.pop();
            }
            if !stack.is_empty() && *n > stack.last().unwrap().1 {
                return true;
            }

            stack.push((*n, current_min));
            current_min = current_min.min(*n);
        }

        false
    }

    // AC
    pub fn find132pattern_2(nums: Vec<i32>) -> bool {
        let mut stack: Vec<(i32, i32)> = vec![];
        let mut current_min = nums[0];

        for n in nums.iter().skip(1) {
            while let Some(stack_last) = stack.last() {
                if *n >= stack_last.0 {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(stack_last) = stack.last() {
                if *n > stack_last.1 {
                    return true;
                }
            }

            stack.push((*n, current_min));
            current_min = current_min.min(*n);
        }

        false
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 4];
    // false
    let case_2 = vec![3, 1, 4, 2];
    // true
    let case_3 = vec![-1, 3, 2, 0];
    // true
    let case_4 = vec![1, 0, 1, -4, -3];
    // false
    let case_5 = vec![3, 5, 0, 3, 4];
    // true

    /*
    println!("case_1: {}", Solution::find132pattern(case_1.clone()));
    println!("case_2: {}", Solution::find132pattern(case_2.clone()));
    println!("case_3: {}", Solution::find132pattern(case_3.clone()));
    println!("case_4: {}", Solution::find132pattern(case_4.clone()));
    println!("case_5: {}", Solution::find132pattern(case_5.clone()));
    */

    println!("case_1: {}", SolutionAns::find132pattern(case_1.clone()));
    println!("case_2: {}", SolutionAns::find132pattern(case_2.clone()));
    println!("case_3: {}", SolutionAns::find132pattern(case_3.clone()));
    println!("case_4: {}", SolutionAns::find132pattern(case_4.clone()));
    println!("case_5: {}", SolutionAns::find132pattern(case_5.clone()));

    println!("case_1: {}", SolutionAns::find132pattern_2(case_1.clone()));
    println!("case_2: {}", SolutionAns::find132pattern_2(case_2.clone()));
    println!("case_3: {}", SolutionAns::find132pattern_2(case_3.clone()));
    println!("case_4: {}", SolutionAns::find132pattern_2(case_4.clone()));
    println!("case_5: {}", SolutionAns::find132pattern_2(case_5.clone()));
}
