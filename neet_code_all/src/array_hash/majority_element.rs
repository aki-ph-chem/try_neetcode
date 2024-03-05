use std::collections::HashMap;

struct Solution {}
impl Solution {
    /// AC
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for v in &nums {
            *map.entry(*v).or_default() += 1;
        }

        let mut majo_idx = nums[0];
        for (idx, num) in map {
            if num > n / 2 {
                majo_idx = idx;
                break;
            }
        }

        majo_idx
    }

    // AC
    pub fn majority_element_2(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut majo_idx = nums[0];
        for v in &nums {
            *map.entry(*v).or_default() += 1;
            if let Some(majo_num) = map.get(v) {
                if *majo_num > n / 2 {
                    majo_idx = *v;
                    break;
                }
            }
        }

        majo_idx
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // time: O(N), space: O(1)
    // Boyer-Moore のアルゴリズム
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut res, mut count) = (0, 0);

        for n in nums {
            if count == 0 {
                res = n;
            }
            count += if n == res { 1 } else { -1 };
        }

        res
    }

    // time: O(N), space: O(N)
    pub fn majority_element_2(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let (mut res, mut max_count) = (0, 0);

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
            res = if *count.get(&num).unwrap() > max_count {
                num
            } else {
                res
            };

            max_count = i32::max(*count.get(&num).unwrap(), max_count);
        }

        res
    }

    // time: O(NlogN), space: O(1)
    pub fn majority_element_3(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}

fn main() {
    let case_1 = vec![3, 2, 3];
    // => 3
    let case_2 = vec![2, 2, 1, 1, 1, 2, 2];
    // => 2

    println!("case_1: {}", Solution::majority_element(case_1.clone()));
    println!("case_1: {}", Solution::majority_element_2(case_1.clone()));
    println!("case_2: {}", Solution::majority_element(case_2.clone()));
    println!("case_2: {}", Solution::majority_element_2(case_2.clone()));

    println!("case_1: {}", SolutionAns::majority_element(case_1.clone()));
    println!(
        "case_1: {}",
        SolutionAns::majority_element_2(case_1.clone())
    );
    println!(
        "case_1: {}",
        SolutionAns::majority_element_3(case_1.clone())
    );
    println!("case_2: {}", SolutionAns::majority_element(case_2.clone()));
    println!(
        "case_2: {}",
        SolutionAns::majority_element_2(case_2.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::majority_element_3(case_2.clone())
    );
}
