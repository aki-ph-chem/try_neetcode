// 解けなかった
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        // dp[i][j] i - 1をスタートして売買を始めた場合のスコア
        let mut dp = vec![vec![0; n]; n];

        //for i in 1..n {
        for i in 1..2 {
            let mut j = i;
            while j < n {
                if prices[j - 1] <= prices[j] {
                    if j + 3 < n {
                        dp[i][j + 3] =
                            std::cmp::max(dp[i][j], dp[i][j - 1] + prices[j] - prices[j - 1]);
                    }
                    j += 3;
                } else {
                    j += 1;
                }
            }
        }

        dp[n - 1][n - 1]
    }

    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut result = 0;

        // i = 1から始める
        let mut i = 1;
        while i < n {
            // i - 1で買って、iで売る
            println!("i = {}", i);
            if prices[i - 1] <= prices[i] {
                result += prices[i] - prices[i - 1];
                // 売買した場合はiを3進める
                i += 3;
            } else {
                // 売買しなかった場合はiを1進める
                i += 1;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // あまりDPっぽくない
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        for x in 1..prices.len() {
            let temp = p1;
            p1 = std::cmp::max(p1 + prices[x] - prices[x - 1], p2);
            p2 = std::cmp::max(temp, p2);
        }

        std::cmp::max(p1, p2)
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 0, 2];
    // 3
    let case_2 = vec![1];
    // 0

    println!("case_1: {}", Solution::max_profit(case_1.clone()));
    println!("case_2: {}", Solution::max_profit(case_2.clone()));

    /*
    println!("case_1: {}", Solution::max_profit_2(case_1.clone()));
    println!("case_2: {}", Solution::max_profit_2(case_2.clone()));
    */

    println!("case_1: {}", SolutionAns::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_profit(case_2.clone()));
}
