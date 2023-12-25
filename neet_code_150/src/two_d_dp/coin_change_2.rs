// 解けなかった
struct Solution {}
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // dp[i][j] i - 1 番目までのコインで
        // j - 1 円まで表現した場合のスコア
        let mut dp = vec![vec![amount + 1; (amount + 1) as usize]; coins.len()];
        dp[0][0] = 0;

        for a in 1..coins.len() {
            for b in 1..=amount {
                if b - coins[a] >= 0 {
                    dp[a][b as usize] = std::cmp::min(
                        dp[a][b as usize],
                        dp[a - 1][(b - coins[a as usize - 1]) as usize] + 1,
                    );
                }
            }
        }

        if dp[coins.len() - 1][amount as usize] == amount + 1 {
            return 0;
        }

        dp[coins.len() - 1][amount as usize]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = amount as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for coin in coins {
            for i in (coin as usize)..=n {
                dp[i] += dp[i - coin as usize];
            }
        }
        *dp.last().unwrap()
    }

    // 別解
    pub fn change_2(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for coin in coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] += dp[i - coin as usize];
            }
        }

        dp[amount as usize]
    }
}

fn main() {
    let case_1 = (5, vec![1, 2, 5]);
    // 5 = 5
    // 5 = 2 + 2 + 1
    // 5 = 2 + 1 + 1 + 1
    // 5 = 1 + 1 + 1 + 1 + 1
    // => 4
    let case_2 = (3, vec![2]);
    // 0
    let case_3 = (10, vec![10]);
    // 1

    println!("case_1: {}", Solution::change(case_1.0, case_1.1.clone()));
    println!("case_2: {}", Solution::change(case_2.0, case_2.1.clone()));
    println!("case_3: {}", Solution::change(case_3.0, case_3.1.clone()));

    println!("case_1: {}", SolutionAns::change(case_1.0, case_1.1.clone()));
    println!("case_2: {}", SolutionAns::change(case_2.0, case_2.1.clone()));
    println!("case_3: {}", SolutionAns::change(case_3.0, case_3.1.clone()));

    println!("case_1: {}", SolutionAns::change_2(case_1.0, case_1.1.clone()));
    println!("case_2: {}", SolutionAns::change_2(case_2.0, case_2.1.clone()));
    println!("case_3: {}", SolutionAns::change_2(case_3.0, case_3.1.clone()));
}
