struct Solution {}
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // 累積和を考える
        // prefix_sum[i]: i - 1番目までの和
        let mut prefix_sum = vec![0; nums.len() + 1];
        prefix_sum[0] = 0;

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        let mut result = -1;
        for k in 0..prefix_sum.len() {
            if k + 1 < prefix_sum.len() {
                let left_sum = prefix_sum[k] - prefix_sum[0];
                let right_sum = prefix_sum[nums.len()] - prefix_sum[k + 1];

                if left_sum == right_sum {
                    result = k as i32;
                    break;
                }
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left_sum = 0;

        for (i, n) in nums.iter().enumerate() {
            let right_sum = total - n - left_sum;
            if left_sum == right_sum {
                return i as i32;
            }
            left_sum += n;
        }

        -1
    }
}

fn main() {
    let case_1 = vec![1, 7, 3, 6, 5, 6];
    // => 3
    let case_2 = vec![1, 2, 3];
    // => -1
    let case_3 = vec![2, 1, -1];
    // => 0
    let case_4 = vec![-1, -1, 0, 0, -1, -1];
    // => 2

    println!("case_1: {:?}", Solution::pivot_index(case_1.clone()));
    println!("case_2: {:?}", Solution::pivot_index(case_2.clone()));
    println!("case_3: {:?}", Solution::pivot_index(case_3.clone()));
    println!("case_4: {:?}", Solution::pivot_index(case_4.clone()));

    println!("case_1: {:?}", SolutionAns::pivot_index(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::pivot_index(case_2.clone()));
    println!("case_3: {:?}", SolutionAns::pivot_index(case_3.clone()));
    println!("case_4: {:?}", SolutionAns::pivot_index(case_4.clone()));
}
