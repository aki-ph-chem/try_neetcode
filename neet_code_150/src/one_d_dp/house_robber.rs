// 解けなかった
// 一度i 番目の配列にアクセスしたら i-1,i+1はアクセスできない
struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![-(1 << 30); n];

        if n == 1 {
            return nums[0];
        }

        dp[0] = nums[0];
        dp[1] = nums[1];

        for i in 2..n {
            for k in 0..(i - 2) {
                dp[i] = std::cmp::max(dp[i - k - 1] + nums[i], dp[i - k] + nums[i]);
            }
        }

        dp[n - 1]
    }

    // 模範解答を見た後で考えた
    // AC
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // dp[i]: i - 1 までに得たコスト
        let mut dp = vec![-(1 << 30); n + 1];

        if n == 1 {
            return nums[0]; 
        }

        dp[0] = 0;
        dp[1] = nums[0];
        for i in 2..n + 1{
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i - 1]);
        }

        dp[n]
    } 
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |loot, money| (loot.1, loot.1.max(loot.0 + money)))
            .1
    }

    // Leet Codeより
    // こっちの方がdpっぽい(自然に感じる)
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        // robbed[i]: i - 1までに得たコスト
        let mut robbed = vec![0; nums.len() + 1];

        robbed[1] = nums[0];
        for i in 1..nums.len() {
            robbed[i + 1] = std::cmp::max(robbed[i], robbed[i - 1] + nums[i]);
        }

        robbed[nums.len()]
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 1];
    let case_2 = vec![2, 7, 9, 3, 1];

    println!("case_1: {}", Solution::rob(case_1.clone()));
    println!("case_2: {}", Solution::rob(case_2.clone()));

    println!("case_1: {}", Solution::rob_2(case_1.clone()));
    println!("case_2: {}", Solution::rob_2(case_2.clone()));

    println!("case_1: {}", SolutionAns::rob(case_1.clone()));
    println!("case_2: {}", SolutionAns::rob(case_2.clone()));

    println!("case_1: {}", SolutionAns::rob_2(case_1.clone()));
    println!("case_2: {}", SolutionAns::rob_2(case_2.clone()));
}
