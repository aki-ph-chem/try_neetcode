use std::collections::HashMap;

struct Solution {}
impl Solution {
    // 縛り無視ならビット演算で解くのが一番カンタン time: O(N), space: O(1)
    // AC
    pub fn single_non_duplicate_bit(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for v in nums {
            result ^= v;
        }

        result
    }

    // 縛り無視その２time: O(N), space: O(N)
    pub fn single_non_duplicate_map(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in nums {
            *map.entry(v).or_default() += 1;
        }

        let mut result = 0;
        for (v, n) in map {
            if n == 1 {
                result = v;
            }
        }

        result
    }

    // 縛りありでは解けなかった
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);
        let mut mid = 0;
        while left <= right {
            mid = left + (right - left) / 2;
            if (mid - left + 1) % 2 == 0 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        nums[mid as usize]
    }
}
// 縛り: time O(logN), space: O(1)

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 2);

        while left <= right {
            let mid_1 = left + (right - left) / 2;
            let mid_2 = mid_1 ^ 1;

            if nums[mid_1 as usize] == nums[mid_2 as usize] {
                left = mid_1 + 1;
            } else {
                right = mid_1 - 1;
            }
        }

        nums[left as usize]
    }
}

fn main() {
    let case_1 = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
    // => 2
    let case_2 = vec![3, 3, 7, 7, 10, 11, 11];
    // => 10
    let case_3 = vec![1, 1, 2];
    // => 2

    println!(
        "case_1: {}",
        Solution::single_non_duplicate_bit(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::single_non_duplicate_bit(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::single_non_duplicate_bit(case_3.clone())
    );

    println!(
        "case_1: {}",
        Solution::single_non_duplicate_map(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::single_non_duplicate_map(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::single_non_duplicate_map(case_3.clone())
    );

    /*
    println!("case_1: {}", Solution::single_non_duplicate(case_1.clone()));
    println!("case_2: {}", Solution::single_non_duplicate(case_2.clone()));
    println!("case_3: {}", Solution::single_non_duplicate(case_3.clone()));
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::single_non_duplicate(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::single_non_duplicate(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::single_non_duplicate(case_3.clone())
    );
}
