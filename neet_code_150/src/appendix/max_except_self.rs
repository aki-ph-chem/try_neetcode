struct Solution {}
impl Solution {
    // O(N^2)
    pub fn max_except_self_sq(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums.len() {
            let mut max_tmp = -(1 << 30);
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                if max_tmp < nums[j] {
                    max_tmp = nums[j];
                }
            }
            result.push(max_tmp);
        }

        result
    }

    // O(N)
    pub fn max_except_self(nums: Vec<i32>) -> Vec<i32> {
        // max_list[i]: iまでの要素のmax
        let mut max_list = vec![0; nums.len()];
        max_list[0] = nums[0];
        for i in 1..nums.len() {
            if max_list[i - 1] <= nums[i] {
                max_list[i] = nums[i];
            } else {
                max_list[i] = max_list[i - 1];
            }
        }

        let mut result = vec![0; nums.len()];
        // resultの末尾
        result[max_list.len() - 1] = max_list[max_list.len() - 2];
        let mut max_right = nums[nums.len() - 1];

        for i in (1..nums.len() - 1).rev() {
            if max_right <= nums[i + 1] {
                max_right = nums[i + 1];
            }

            if max_list[i - 1] <= max_right {
                result[i] = max_right;
            } else {
                result[i] = max_list[i - 1];
            }
            // resultの頭
            result[0] = max_right;
        }

        result
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 4];
    let case_2 = vec![5, 2, 13, 4, 16, 8, 12, 9];

    println!("case_1: {:?}", Solution::max_except_self_sq(case_1.clone()));
    println!("case_1: {:?}", Solution::max_except_self(case_1.clone()));
    println!("case_2: {:?}", Solution::max_except_self_sq(case_2.clone()));
    println!("case_2: {:?}", Solution::max_except_self(case_2.clone()));
}
