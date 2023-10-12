struct Solution {}
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_nums = nums.len();
        let mut res = vec![];
        for i in 0..len_nums {
            for j in (i+1)..len_nums {
                if nums[i] + nums[j] == target {
                    res.push(i as i32);
                    res.push(j as i32);
                }
            }
        }
        res.sort();
        res[0] += 1;
        res[1] += 1;
        res
    }
}

// 模範解答
use std::cmp::Ordering::{Equal, Greater, Less};
struct SolutionAns {}
impl SolutionAns {
  pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            match (numbers[l] + numbers[r]).cmp(&target) {
                Less => l += 1,
                Greater => r -= 1,
                Equal => return vec![l as i32 + 1, r as i32 + 1],
            }
        }
        unreachable!("Test did not follow the constraints!")
    }
}

fn main() {
    let case_1 = (vec![2, 7, 11, 15], 9);
    let case_2 = (vec![2, 3, 4], 6);
    let case_3 = (vec![-1, 0], -1);

    println!("case_1: {:?}", Solution::two_sum(case_1.clone().0, case_1.clone().1));
    println!("case_2: {:?}", Solution::two_sum(case_2.clone().0, case_2.clone().1));
    println!("case_3: {:?}", Solution::two_sum(case_3.clone().0, case_3.clone().1));

    println!("case_1: {:?}", SolutionAns::two_sum(case_1.clone().0, case_1.clone().1));
    println!("case_2: {:?}", SolutionAns::two_sum(case_2.clone().0, case_2.clone().1));
    println!("case_3: {:?}", SolutionAns::two_sum(case_3.clone().0, case_3.clone().1));
}
