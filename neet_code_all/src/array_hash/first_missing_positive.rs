// 縛り: メモリ O(1), 時間 O(N)

// hit1: まずは追加のメモリのことを考えずに考えてみよう
// hit2: ダブった値や負の値は無視
// hit3: O(2n) = O(n)

use std::collections::HashMap;

// 解けなかった
struct Solution;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let min_pos = nums.iter().fold(i32::MAX, |mut min, x| {
            if *x > 0 {
                min = min.min(*x);
            }
            min
        });
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(vec_n) = map.get_mut(&n) {
                vec_n.push(i);
            } else {
                map.insert(*n, vec![i]);
            }
        }
        println!("map: {:?}", map);

        1
    }
}

// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            if i as i32 + 1 == nums[i] {
                continue;
            }
            let mut x = nums[i];
            while x >= 1 && x as usize <= nums.len() && x != nums[x as usize - 1] {
                let tmp = nums[x as usize - 1];
                nums[x as usize - 1] = x;
                x = tmp;
            }
        }

        for i in 0..nums.len() {
            if i as i32 + 1 != nums[i] {
                return i as i32 + 1;
            }
        }

        nums.len() as i32 + 1
    }
}

fn main() {
    let case_1 = vec![1, 2, 0];
    // => 3
    let case_2 = vec![3, 4, -1, 1];
    // => 2
    let case_3 = vec![7, 8, 9, 11, 12];
    // => 1

    /*
    println!(
        "case_1: {}",
        Solution::first_missing_positive(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::first_missing_positive(case_2.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::first_missing_positive(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::first_missing_positive(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::first_missing_positive(case_3.clone())
    );
}
