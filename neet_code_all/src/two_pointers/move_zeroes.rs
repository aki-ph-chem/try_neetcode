use std::collections::VecDeque;

// inplace 縛りあり
struct Solution {}
impl Solution {
    // AC
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            if nums[l] == 0 {
                for i in l..nums.len() - 1 {
                    nums[i] = nums[i + 1];
                }
                nums[r] = 0;
                r -= 1;
            } else {
                l += 1;
            }
        }
    }

    // 縛りを無視するならば...
    // AC
    pub fn move_zeroes_out_place(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut stack = VecDeque::new();

        for v in nums.iter() {
            if *v != 0 {
                stack.push_back(*v);
            }
        }

        let mut i = 0;
        while let Some(stack_v) = stack.pop_front() {
            nums[i] = stack_v;
            i += 1;
        }
        while i < n {
            nums[i] = 0;
            i += 1;
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;

        for r in 0..nums.len() {
            if nums[r] != 0 {
                nums.swap(left, r);
                left += 1;
            }
        }
    }
}

fn main() {
    let case_1 = vec![0, 1, 0, 3, 12];
    let case_2 = vec![0];
    let case_3 = vec![0, 0, 1];

    let mut res_1 = case_1.clone();
    Solution::move_zeroes(&mut res_1);
    println!("res_1 :{:?}", res_1);

    let mut res_2 = case_2.clone();
    Solution::move_zeroes(&mut res_2);
    println!("res_2 :{:?}", res_2);

    let mut res_3 = case_3.clone();
    Solution::move_zeroes(&mut res_3);
    println!("res_3 :{:?}", res_3);

    let mut res_1 = case_1.clone();
    Solution::move_zeroes_out_place(&mut res_1);
    println!("res_1 :{:?}", res_1);

    let mut res_2 = case_2.clone();
    Solution::move_zeroes_out_place(&mut res_2);
    println!("res_2 :{:?}", res_2);

    let mut res_3 = case_3.clone();
    Solution::move_zeroes_out_place(&mut res_3);
    println!("res_3 :{:?}", res_3);

    let mut res_1 = case_1.clone();
    SolutionAns::move_zeroes(&mut res_1);
    println!("res_1 :{:?}", res_1);

    let mut res_2 = case_2.clone();
    SolutionAns::move_zeroes(&mut res_2);
    println!("res_2 :{:?}", res_2);

    let mut res_3 = case_3.clone();
    SolutionAns::move_zeroes(&mut res_3);
    println!("res_3 :{:?}", res_3);
}
