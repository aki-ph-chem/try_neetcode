use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};

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
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Less => l += 1,
                    Greater => r -= 1,
                    Equal => {
                        ans.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        while nums[l] == nums[l - 1] && l < r {
                            l += 1;
                        }
                        r -= 1;
                        while nums[r] == nums[r + 1] && l < r {
                            r -= 1;
                        }
                    }
                }
            }
        }

        ans
    }

    // 部分的な別解(matchがif elseになっただけだが..)
    // AC
    pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = vec![];

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if 0 < sum {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    ans.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                    right -= 1;
                    while nums[right] == nums[right + 1] && left < right {
                        right -= 1;
                    }
                }
            }
        }

        ans
    }
}

// C++の模範解答
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let n = nums.len();
        if n < 3 {
            result.push(vec![]);
            return result;
        }

        nums.sort();
        for i in 0..(n - 2) {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let (mut j, mut k) = (i + 1, n - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {
                    result.push(vec![nums[i], nums[j], nums[k]]);

                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    j += 1;

                    while j < k && nums[k - 1] == nums[k] {
                        k -= 1;
                    }
                    k -= 1;
                }
            }
        }

        result
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

    println!("case_1: {:?}", SolutionAnsCpp::three_sum(case_1.clone()));
    println!("case_2: {:?}", SolutionAnsCpp::three_sum(case_2.clone()));
    println!("case_3: {:?}", SolutionAnsCpp::three_sum(case_3.clone()));
    println!("case_4: {:?}", SolutionAnsCpp::three_sum(case_4.clone()));
}
