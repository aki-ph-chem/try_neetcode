struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_pro = -(1 << 30);
        let (mut idx_l, mut idx_r) = (0, prices.len() - 1);

        while idx_l < idx_r {
            if max_pro < prices[idx_r] - prices[idx_l] {
                max_pro = prices[idx_r] - prices[idx_l];
            }

            if prices[idx_l] < prices[idx_r] {
                idx_l += 1;
            } else {
                idx_r -= 1;
            }
        }

        if max_pro < 0 {
            0
        } else {
            max_pro
        }
    }

    // O(N^2)でTLE
    pub fn max_profit_sq(prices: Vec<i32>) -> i32 {
        let mut max_pro = -(1 << 30);
        for i in 0..prices.len() {
            for j in (i + 1)..prices.len() {
                if max_pro < prices[j] - prices[i] {
                    max_pro = prices[j] - prices[i];
                }
            }
        }
        if max_pro < 0 {
            0
        } else {
            max_pro
        }
    }
}

struct SolutionAns {}
impl SolutionAns {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 1;
        let mut max_profit = 0;

        while r < prices.len() {
            if prices[l] < prices[r] {
                let profit = prices[r] - prices[l];
                max_profit = std::cmp::max(profit, max_profit);
            } else {
                l = r;
            }
            r += 1;
        }

        max_profit
    }
}

// C++の模範解答
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut max_p, mut l, mut r) = (0, 0, 0);

        while r < prices.len() {
            if prices[r] > prices[l] {
                max_p = max_p.max(prices[r] - prices[l]);
            } else {
                l = r;
            }
            r += 1;
        }

        max_p
    }
}

// 部分的な別解
struct SolutionLatter {}
impl SolutionLatter {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut result = 0;

        while right < prices.len() {
            while right < prices.len() && prices[left] < prices[right] {
                result = result.max(prices[right] - prices[left]);
                right += 1;
            }
            left = right;
            right += 1;
        }

        result
    }
}

fn main() {
    let case_1 = vec![7, 1, 5, 3, 6, 4];
    let case_2 = vec![7, 6, 4, 3, 1];

    println!("case_1: {}", Solution::max_profit_sq(case_1.clone()));
    println!("case_2: {}", Solution::max_profit_sq(case_2.clone()));

    println!("case_1: {}", Solution::max_profit(case_1.clone()));
    println!("case_2: {}", Solution::max_profit(case_2.clone()));

    println!("case_1: {}", SolutionAns::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_profit(case_2.clone()));

    println!("case_1: {}", SolutionAnsCpp::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::max_profit(case_2.clone()));

    println!("case_1: {}", SolutionLatter::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionLatter::max_profit(case_2.clone()));
}
