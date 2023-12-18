// 解けなかった
struct Solution {}
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let n = coins.len();
        let mut dp = vec![1 << 30; n];
        coins.sort_by(|a, b| b.cmp(a));

        dp[0] = amount / coins[0];
        for i in 1..n {
            dp[i] = std::cmp::min(dp[i - 1] + (amount / coins[i]), amount % coins[i]);
        }

        dp[n - 1]
    }

    // 貪欲法は常にはうまくいかない
    pub fn coin_change_greedy(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        let mut result = 0;
        coins.sort_by(|a, b| b.cmp(a));

        for v in coins {
            result += amount / v;
            amount = amount % v;
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut change = vec![amount + 1; (amount + 1) as usize];
        change[0] = 0;
        for a in 1..=amount {
            for c in &coins {
                if a - c >= 0 {
                    change[a as usize] = change[a as usize].min(1 + change[(a - c) as usize]);
                }
            }
        }
        if change[amount as usize] == amount + 1 {
            return -1;
        }
        change[amount as usize]
    }
}

fn main() {
    let case_1 = (vec![1, 2, 5], 11);
    // 5 + 5 + 1 => 3
    let case_2 = (vec![2], 3);
    // -1
    let case_3 = (vec![1], 0);
    // 0
    let case_4 = (vec![1, 100, 500], 1200);
    // 500 * 2 + 100 *2 => 4

    println!(
        "case_1: {}",
        Solution::coin_change(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::coin_change(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::coin_change(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::coin_change(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::coin_change(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::coin_change(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::coin_change(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAns::coin_change(case_4.0.clone(), case_4.1)
    );
}
