// 解けなかった
// 残りのn個の和まではかんたんだが...
struct Solution;
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let mut sum_m = 0;
        for r in rolls {
            sum_m += r;
        }

        let sum_n = mean * (m + n) - sum_m;
        println!("sum_n: {sum_n}");

        let mut result = vec![0; n as usize];
        if sum_n % n == 0 {
            for i in 0..n as usize {
                result[i] = sum_n / n;
            }
        } else {
        }

        result
    }
}

// Ktolinの模範解答より
struct SolutionAnsKotlin;
impl SolutionAnsKotlin {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let (m, mut n) = (rolls.len() as i32, n);
        let mut n_total = (mean * (n + m)) - rolls.iter().sum::<i32>();

        if n_total < n || n_total > n * 6 {
            return vec![];
        }

        let mut result = vec![];
        while n_total > 0 {
            let dice = std::cmp::min(n_total - n + 1, 6);
            result.push(dice);
            n_total -= dice;
            n -= 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![3, 2, 4, 3], 4, 2);
    // [6, 6]
    let case_2 = (vec![1, 5, 6], 3, 4);
    // [2, 3, 2, 2]
    let case_3 = (vec![1, 2, 3, 4], 6, 4);
    // []

    /*
    println!(
        "case_1: {:?}",
        Solution::missing_rolls(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {:?}",
        Solution::missing_rolls(case_2.0.clone(), case_2.1, case_2.2)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsKotlin::missing_rolls(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsKotlin::missing_rolls(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsKotlin::missing_rolls(case_3.0.clone(), case_3.1, case_3.2)
    );
}
