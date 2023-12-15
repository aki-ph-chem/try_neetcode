// i番目にアクセスするとi-1,i+1にアクセスできない
// また配列の最後の要素は配列の最初の要素と隣接する(環状とする)
struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // dp[i]: i - 1 までに得たコスト
        let mut dp = vec![-(1 << 30); n + 1];

        dp[0] = 0;
        dp[1] = nums[0];
        for i in 2..n + 1 {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i - 1]);
        }

        std::cmp::max(dp[n], dp[n - 1])
    }

    // AC
    // C++の模範解答をもとに考えた
    // なぜかC++のものより高速&省メモリ
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return nums[0];
        } else if n == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }

        // dp_1[i],dp_2[i]: i - 1までのコスト
        let mut dp_1 = vec![-(1 << 30); n + 1];
        let mut dp_2 = vec![-(1 << 30); n + 1];

        // dp_1: nums[0] スタート
        dp_1[0] = 0;
        dp_1[1] = nums[0];
        // dp_2: nums[1] スタート
        dp_2[0] = 0;
        dp_2[1] = 0;
        dp_2[2] = nums[1];

        for i in 2..(n + 1) {
            if i <= n - 1 {
                dp_1[i] = std::cmp::max(dp_1[i - 1], dp_1[i - 2] + nums[i - 1]);
            }

            if i > 2 {
                dp_2[i] = std::cmp::max(dp_2[i - 1], dp_2[i - 2] + nums[i - 1]);
            }
        }

        std::cmp::max(dp_1[n - 1], dp_2[n])
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();

        return match l {
            0 => 0,
            1 => nums[0],
            _ => rob_house(&nums[1..]).max(rob_house(&nums[0..l - 1])),
        };

        fn rob_house(nums: &[i32]) -> i32 {
            nums.iter()
                .fold((0, 0), |loot, money| (loot.1, loot.1.max(loot.0 + money)))
                .1
        }
    }
}

fn main() {
    let case_1 = vec![2, 3, 2]; //3
    let case_2 = vec![1, 2, 3, 1]; //4
    let case_3 = vec![1, 2, 3]; //3

    println!("case_1: {}", Solution::rob(case_1.clone()));
    println!("case_2: {}", Solution::rob(case_2.clone()));
    println!("case_3: {}", Solution::rob(case_3.clone()));

    println!("case_1: {}", SolutionAns::rob(case_1.clone()));
    println!("case_2: {}", SolutionAns::rob(case_2.clone()));
    println!("case_3: {}", SolutionAns::rob(case_3.clone()));

    println!("case_1: {}", Solution::rob_2(case_1.clone()));
    println!("case_2: {}", Solution::rob_2(case_2.clone()));
    println!("case_3: {}", Solution::rob_2(case_3.clone()));
}
