use std::collections::HashSet;

struct Solution {}
impl Solution {
    // AC
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::new();
        let mut result = vec![];
        for v in &nums {
            set.insert(*v);
        }

        let n = nums.len() as i32;
        for i in 1..=n {
            if !set.contains(&i) {
                result.push(i);
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let j = nums[i].abs() - 1;
            nums[j as usize] = -1 * nums[j as usize].abs();
        }

        let mut result = vec![];
        for (idx, number) in nums.iter().enumerate() {
            if *number > 0 {
                result.push((idx + 1) as i32);
            }
        }

        result
    }
}

fn main() {
    let case_1 = vec![4, 3, 2, 7, 8, 2, 3, 1];
    // => [5, 6]
    let case_2 = vec![1, 1];
    // => [2]

    println!(
        "case_1: {:?}",
        Solution::find_disappeared_numbers(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_disappeared_numbers(case_2.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAns::find_disappeared_numbers(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::find_disappeared_numbers(case_2.clone())
    );
}
