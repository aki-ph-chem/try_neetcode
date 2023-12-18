/*

*** encode ***
'A' -> "1"
'B' -> "2"
...
'Z' -> "26"

*** decode **
"1" -> 'A'
...
"26" -> 'Z'

*/

// 解けなかった
struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let length = s.len();
        let char_list: Vec<char> = s.chars().collect();

        // i番目の次に仕切りをいれた場合のスコア
        let mut dp = vec![0; length];
        if char_list[0] == '0' {
            return 0;
        }

        dp[0] = 1;

        for i in 0..length {
            if char_list[i] != '0' {
                dp[i + 1] = dp[i] + 1;
            }

            if i + 2 < length {
                if let Ok(num) = s[i..(i + 1)].parse::<i32>() {
                    if 10 <= num && num <= 26 {
                        dp[i + 2] = dp[i] + 1;
                    }
                }
            }
        }

        dp[length - 1]
    }
}

// C++の模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn num_decodings(s: String) -> i32 {
        let char_list: Vec<char> = s.chars().collect();
        if char_list[0] == '0' {
            return 0;
        }

        let n = s.len();

        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            // 1桁の場合
            if let Ok(ones) = &s[(i - 1)..i].parse::<i32>() {
                if *ones >= 1 && *ones <= 9 {
                    dp[i] += dp[i - 1];
                }
            }

            // 2桁の場合
            if let Ok(tens) = &s[(i - 2)..i].parse::<i32>() {
                if *tens >= 10 && *tens <= 26 {
                    dp[i] += dp[i - 2];
                }
            }
        }

        dp[n]
    }
}

fn main() {
    let case_1 = "12".to_string();
    // "AB"(1,2) or "L"(12) => 2
    let case_2 = "226".to_string();
    // "BZ"(2,26) or "VF"(22, 6)
    // or "BBF" (2,2,6) => 3
    let case_3 = "06".to_string();
    // 0
    let case_4 = "10".to_string();
    // "J" (10) => 1

    /*
    println!("case_1: {}", Solution::num_decodings(case_1.clone()));
    println!("case_2: {}", Solution::num_decodings(case_2.clone()));
    println!("case_3: {}", Solution::num_decodings(case_3.clone()));
    println!("case_4: {}", Solution::num_decodings(case_4.clone()));
    */

    println!("case_1: {}", SolutionAns::num_decodings(case_1.clone()));
    println!("case_2: {}", SolutionAns::num_decodings(case_2.clone()));
    println!("case_3: {}", SolutionAns::num_decodings(case_3.clone()));
    println!("case_4: {}", SolutionAns::num_decodings(case_4.clone()));
}
