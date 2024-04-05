// 解けなかった
struct Solution {}
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut pair = vec![];
        for left in 0..nums.len() {
            for right in (left + 1..nums.len()).rev() {}
        }

        let mut result = 0;
        for p in pair {
            result = result.max(p);
        }
        result
    }
}

// Kotlinの模範解答より(C++,Pythonもなかったので..)
struct SolutionAnsKotlin {}
impl SolutionAnsKotlin {
    // AC
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let n = nums.len() as i32;

        let is_good = |x: i32| {
            let mut i = 0;
            let mut count = 0;
            while i < n - 1 {
                if nums[i as usize + 1] - nums[i as usize] <= x {
                    count += 1;
                    i += 2;
                } else {
                    i += 1;
                }

                if count == p {
                    return true;
                }
            }

            false
        };

        let (mut left, mut right) = (0, i32::MAX);
        while left < right {
            let mid = left + (right - left) / 2;
            if is_good(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    let case_1 = (vec![10, 1, 2, 7, 1, 3], 2);
    // => 1
    let case_2 = (vec![4, 2, 1, 2], 1);
    // => 0

    println!(
        "case_1: {}",
        SolutionAnsKotlin::minimize_max(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::minimize_max(case_2.0.clone(), case_2.1)
    );
}
