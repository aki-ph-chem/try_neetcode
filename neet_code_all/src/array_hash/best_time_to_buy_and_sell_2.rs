// 解けなかった
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut max_p, mut left, mut right) = (0, 0, 0);

        while right < prices.len() {
            if prices[right] > prices[left] {
                max_p = max_p.max(max_p + prices[right] - prices[left]);
            } else {
                left = right;
            }
            right += 1;
        }

        max_p
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut buy = i32::MAX;

        for p in prices {
            buy = buy.min(p - result);
            result = result.max(p - buy);
        }

        result
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result += prices[i] - prices[i - 1];
            }
        }

        result
    }
}

fn main() {
    let case_1 = vec![7, 1, 5, 3, 6, 4];
    // => 7
    let case_2 = vec![1, 2, 3, 4, 5];
    // => 4
    let case_3 = vec![7, 6, 4, 3, 1];
    // => 0

    println!("case_1: {}", Solution::max_profit(case_1.clone()));
    println!("case_2: {}", Solution::max_profit(case_2.clone()));
    println!("case_3: {}", Solution::max_profit(case_3.clone()));

    println!("case_1: {}", SolutionAns::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_profit(case_2.clone()));
    println!("case_3: {}", SolutionAns::max_profit(case_3.clone()));

    println!("case_1: {}", SolutionAnsCpp::max_profit(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::max_profit(case_2.clone()));
    println!("case_3: {}", SolutionAnsCpp::max_profit(case_3.clone()));
}
