struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // dp[i]: iまでのスコア
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = 1;

        for a in 1..n {}

        dp[n - 1]
    }

    // 全探索
    // case_2に対応できない
    pub fn length_of_lis_ex(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;

        for a in 0..n {
            let mut res_tmp = 1;
            let mut tmp = nums[a];

            for b in (a + 1)..n {
                if tmp < nums[b] {
                    tmp = nums[b];

                    res_tmp += 1;
                    println!("(a,b) = ({},{}), res_tmp = {}", a, b, res_tmp);
                }
            }

            res = res.max(res_tmp);
        }

        res
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // こっちのほうが速い(bin searchのため?)
    // dpじゃない?
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = vec![];
        for &num in &nums {
            match tails.binary_search(&num) {
                Ok(_) => {}
                Err(i) => {
                    if i == tails.len() {
                        tails.push(num);
                    } else {
                        tails[i] = num;
                    }
                }
            }
        }

        tails.len() as i32
    }

    // C++の模範解答より
    // AC
    pub fn length_of_lis_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut result = 1;

        for i in 1..n {
            for j in 0..i{
                if nums[j] < nums[i] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
            result = std::cmp::max(result, dp[i]);
        }

        result
    }
}

fn main() {
    let case_1 = vec![10, 9, 2, 5, 3, 7, 101, 18];
    // 4
    let case_2 = vec![0, 1, 0, 3, 2, 3];
    // 4
    let case_3 = vec![7, 7, 7, 7, 7, 7, 7];
    // 1

    /*
    println!("case_1: {}", Solution::length_of_lis_ex(case_1.clone()));
    println!("case_2: {}", Solution::length_of_lis_ex(case_2.clone()));
    println!("case_3: {}", Solution::length_of_lis_ex(case_3.clone()));
    */

    println!("case_1: {}", SolutionAns::length_of_lis(case_1.clone()));
    println!("case_2: {}", SolutionAns::length_of_lis(case_2.clone()));
    println!("case_3: {}", SolutionAns::length_of_lis(case_3.clone()));

    println!("case_1: {}", SolutionAns::length_of_lis_2(case_1.clone()));
    println!("case_2: {}", SolutionAns::length_of_lis_2(case_2.clone()));
    println!("case_3: {}", SolutionAns::length_of_lis_2(case_3.clone()));
}
