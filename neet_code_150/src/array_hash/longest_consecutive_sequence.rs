use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}
impl Solution {
    // 出来なかった
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut result = 1;
        for i in 0..nums.len() {}
        result
    }

    // そもそもダメ
    // O(N^2)だからダメ
    fn n_sq(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut result = 1;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] - nums[j] == 1 {
                    result += 1;
                }
            }
        }
        result
    }

    // そもそも間違い
    // O(NlongN)かかるから解答としてはダメ
    fn use_sort(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut nums_sort = nums;
        nums_sort.sort();

        let mut result = 1;
        for i in 0..(nums_sort.len() - 1) {
            if nums_sort[i + 1] - nums_sort[i] == 1 {
                result += 1;
            }
        }
        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());

        let mut max_cnt = 0;

        for n in &set {
            if !set.contains(&(n - 1)) {
                let mut next = n + 1;
                let mut cnt = 1;
                while set.contains(&next) {
                    cnt += 1;
                    next += 1;
                }

                max_cnt = max_cnt.max(cnt);
            }
        }

        max_cnt
    }
}

fn main() {
    let case_1 = vec![100, 4, 200, 1, 3, 2];
    let case_2 = vec![100, 8, 20, 10, 5, 2];
    let case_3 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let case_4: Vec<i32> = vec![];
    let case_5 = vec![100, 8, 10, 7, 5, 6];
    let case_6 = vec![0, 0, -1];

    println!("use sort");
    println!("case_1: {}", Solution::use_sort(case_1.clone()));
    println!("case_2: {}", Solution::use_sort(case_2.clone()));
    println!("case_3: {}", Solution::use_sort(case_3.clone()));
    println!("case_4: {}", Solution::use_sort(case_4.clone()));
    println!("case_5: {}", Solution::use_sort(case_5.clone()));
    println!("case_6: {}", Solution::use_sort(case_6.clone()));

    println!("O(N^2)");
    println!("case_1: {}", Solution::n_sq(case_1.clone()));
    println!("case_2: {}", Solution::n_sq(case_2.clone()));
    println!("case_3: {}", Solution::n_sq(case_3.clone()));
    println!("case_4: {}", Solution::n_sq(case_4.clone()));
    println!("case_5: {}", Solution::n_sq(case_5.clone()));
    println!("case_6: {}", Solution::n_sq(case_6.clone()));

    println!("ans");
    println!(
        "case_1: {}",
        SolutionAns::longest_consecutive(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::longest_consecutive(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::longest_consecutive(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAns::longest_consecutive(case_4.clone())
    );
    println!(
        "case_5: {}",
        SolutionAns::longest_consecutive(case_5.clone())
    );
    println!(
        "case_6: {}",
        SolutionAns::longest_consecutive(case_6.clone())
    );
}
