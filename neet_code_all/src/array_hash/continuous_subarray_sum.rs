use std::collections::HashMap;

// 解けなかった
struct Solution {}
impl Solution {
    // TLE time: O(N^2)
    pub fn check_subarray_sum_sq(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        //println!("prefix_sum: {:?}", prefix_sum);
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let sum_subarray = prefix_sum[j + 1] - prefix_sum[i];
                if sum_subarray % k == 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() == 1 {
            return false;
        }

        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        //println!("prefix_sum: {:?}", prefix_sum);

        let mut map = HashMap::new();
        for p in prefix_sum {
            let diff = p % k;

            if let Some(v) = map.get(&diff) {
                if p > *v {
                    return true;
                }
            } else {
                map.insert(diff, p);
            }
        }

        false
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut sum = 0;

        for (i, num) in nums.iter().enumerate() {
            sum += num;
            if let Some(prev_idx) = map.get(&(sum % k)) {
                if i as i32 - prev_idx >= 2 {
                    return true;
                }
            } else {
                map.insert(sum % k, i as i32);
            }
        }

        false
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut sum = 0;

        for (idx, n) in nums.iter().enumerate() {
            sum += n;
            if k != 0 {
                sum %= k;
            }

            if let Some(prev_idx) = map.get(&sum) {
                if idx as i32 - prev_idx > 1 {
                    return true;
                }
            } else {
                map.insert(sum, idx as i32);
            }
        }

        false
    }
}

fn main() {
    let case_1 = (vec![23, 2, 6, 4, 7], 6);
    // => true
    let case_2 = (vec![23, 2, 6, 4, 7], 13);
    // => false
    let case_3 = (vec![1, 0], 2);
    // => false
    let case_4 = (vec![23, 2, 4, 6, 6], 7);
    // => true
    let case_5 = (vec![5, 0, 0, 0], 3);
    // => true

    println!(
        "case_1: {}",
        Solution::check_subarray_sum_sq(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::check_subarray_sum_sq(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::check_subarray_sum_sq(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::check_subarray_sum_sq(case_4.0.clone(), case_4.1)
    );
    /*
    println!(
        "case_1: {}",
        Solution::check_subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::check_subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::check_subarray_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::check_subarray_sum(case_4.0.clone(), case_4.1)
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::check_subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::check_subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::check_subarray_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAns::check_subarray_sum(case_4.0.clone(), case_4.1)
    );
    println!(
        "case_5: {}",
        SolutionAns::check_subarray_sum(case_5.0.clone(), case_5.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::check_subarray_sum(case_3.0.clone(), case_3.1)
    );
}
