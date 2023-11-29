// 最小部分列

// 初見では解けなかった
struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        3
    }
}

struct SolutionAns {}
impl SolutionAns {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut sum = 0;

        for n in nums {
            if sum < 0 {
                sum = 0;
            }

            sum += n;
            res = res.max(sum);
        }

        res
    }
}

fn main() {
    let case_1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let case_2 = vec![1];
    let case_3 = vec![5, 4, -1, 7, 8];

    println!("case_1: {}", SolutionAns::max_sub_array(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_sub_array(case_2.clone()));
    println!("case_3: {}", SolutionAns::max_sub_array(case_3.clone()));
}
