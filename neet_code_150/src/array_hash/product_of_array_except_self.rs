struct Solution {}
impl Solution {
    // これだと TLE
    // O(N^2)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                result[i] *= nums[j];
            }
        }
        result
    }

    // O(N^2)を改善する方法は出来なかった😭
    /*
    pub fn product_except_self_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        let mut acm = vec![1; nums.len()];
        for i in 0..nums.len() {
            acm[i] *= nums[i];
        }
        let prod_total =  acm[nums.len() - 1];
        println!("prod_total: {}", prod_total);

        for i in 0..nums.len() {
            if nums[i] !=0 {
                result[i] = prod_total / nums[i];
            } else {
                result[i] = acm[i-1];
                for j in i..nums.len() {
                    result[i] *= nums[j];
                }
            }
        }
        result
    }
    */
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];

        for i in 1..nums.len() {
            res[i] = nums[i - 1] * res[i - 1];
        }

        let mut right = 1;

        for (i, n) in res.iter_mut().enumerate().rev() {
            *n = *n * right;
            right *= nums[i];
        }

        res
    }
}

// 後で時間を置いて解いたときの別解
struct SolutionLatter {}
impl SolutionLatter {
    // AC
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // より累積的な積として
        let n = nums.len();
        let mut prefix = vec![1; n + 1];
        for i in 0..n {
            prefix[i + 1] = nums[i] * prefix[i];
        }
        //println!("prefix: {:?}", prefix);

        let mut result = vec![0; n];
        let mut acc = 1;
        for i in (0..n).rev() {
            result[i] = acc * prefix[i];
            acc *= nums[i];
        }

        result
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 4];
    let case_2 = vec![-1, 1, 0, -3, 3];

    let res_1 = Solution::product_except_self(case_1.clone());
    println!("case_1: {:?}", res_1);

    let res_2 = Solution::product_except_self(case_2.clone());
    println!("case_2: {:?}", res_2);

    let res_1 = SolutionAns::product_except_self(case_1.clone());
    println!("case_1: {:?}", res_1);

    let res_2 = SolutionAns::product_except_self(case_2.clone());
    println!("case_2: {:?}", res_2);

    println!(
        "case_1: {:?}",
        SolutionLatter::product_except_self(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionLatter::product_except_self(case_2.clone())
    );
}
