use std::collections::VecDeque;

struct Solution {}
impl Solution {
    // TLE
    // O((N - k)k)
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![];

        for i in 0..(n + 1 - k as usize) {
            let mut max_t = i32::MIN;
            for j in i..(i + k as usize) {
                max_t = max_t.max(nums[j]);
            }
            result.push(max_t);
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut output = vec![];
        let mut q: VecDeque<usize> = VecDeque::new();

        let (mut l, mut r) = (0, 0);

        while r < nums.len() {
            while !q.is_empty() && nums[r] > nums[*q.back().unwrap()] {
                q.pop_back();
            }

            q.push_back(r);

            if l > *q.front().unwrap() {
                q.pop_front();
            }

            if r + 1 >= k as usize {
                output.push(nums[*q.front().unwrap()]);
                l += 1;
            }

            r += 1;
        }

        output
    }
}

// VecDeque<(usize, i32)>で書ければすこしスマートなのだが..(まだ途中)
struct SolutionRe {}
impl SolutionRe {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut q: VecDeque<(usize, i32)> = VecDeque::new();
        let (mut l, mut r) = (0, 0);

        while r < nums.len() {
            while !q.is_empty() && q.back().unwrap().1 < nums[r] {
                q.pop_back();
            }

            q.push_back((r, nums[r]));

            if l > q.front().unwrap().0 {
                q.pop_back();
            }

            if r + 1 >= k as usize {
                result.push(q.front().unwrap().1);
                l += 1;
            }

            r += 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    // => [3,3,5,6,7]
    let case_2 = (vec![1], 1);
    // => [1]

    println!(
        "case_1: {:?}",
        Solution::max_sliding_window(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::max_sliding_window(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAns::max_sliding_window(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::max_sliding_window(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionRe::max_sliding_window(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionRe::max_sliding_window(case_2.0.clone(), case_2.1.clone())
    );
}
