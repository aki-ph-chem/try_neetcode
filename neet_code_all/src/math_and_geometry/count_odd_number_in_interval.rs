struct Solution;
impl Solution {
    // TLE
    pub fn count_odds_linear(low: i32, high: i32) -> i32 {
        let mut result = 0;
        for i in low..=high {
            if i % 2 == 1 {
                result += 1;
            }
        }

        result
    }

    // AC
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let num = high - low + 1;
        if num % 2 == 0 {
            return num / 2;
        }

        if low % 2 == 0 {
            num / 2
        } else {
            num / 2 + 1
        }
    }

    // AC
    pub fn count_odds_2(low: i32, high: i32) -> i32 {
        let num = high - low + 1;
        if num % 2 == 0 || low % 2 == 0 {
            return num / 2;
        }

        num / 2 + 1
    }
}

// Kotlinの模範解答より
struct SolutoinAnsKotlin;
impl SolutoinAnsKotlin {
    // AC
    pub fn count_odds_1(low: i32, high: i32) -> i32 {
        let mut result = 0;
        result += (high - low) / 2;
        if low % 2 == 1 || high % 2 == 1 {
            result += 1;
        }

        result
    }

    // AC
    pub fn count_odds_2(low: i32, high: i32) -> i32 {
        (high + 1) / 2 - low / 2
    }
}

fn main() {
    let case_1 = (3, 7);
    let case_2 = (8, 10);

    println!(
        "case_1: {}",
        Solution::count_odds_linear(case_1.0, case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::count_odds_linear(case_2.0, case_2.1)
    );

    println!("case_1: {}", Solution::count_odds(case_1.0, case_1.1));
    println!("case_2: {}", Solution::count_odds(case_2.0, case_2.1));

    println!("case_1: {}", Solution::count_odds_2(case_1.0, case_1.1));
    println!("case_2: {}", Solution::count_odds_2(case_2.0, case_2.1));

    println!(
        "case_1: {}",
        SolutoinAnsKotlin::count_odds_1(case_1.0, case_1.1)
    );
    println!(
        "case_2: {}",
        SolutoinAnsKotlin::count_odds_1(case_2.0, case_2.1)
    );

    println!(
        "case_1: {}",
        SolutoinAnsKotlin::count_odds_2(case_1.0, case_1.1)
    );
    println!(
        "case_2: {}",
        SolutoinAnsKotlin::count_odds_2(case_2.0, case_2.1)
    );
}
