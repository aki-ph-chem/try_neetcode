struct Solution {}
impl Solution {
    // AC
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len_1 = text1.len();
        let len_2 = text2.len();
        let char_vec_1: Vec<char> = text1.chars().collect();
        let char_vec_2: Vec<char> = text2.chars().collect();

        // dp[i][j]: i - 1, j - 1までのスコア
        let mut dp = vec![vec![0; len_2 + 1]; len_1 + 1];

        for i in 1..(len_1 + 1) {
            for j in 1..(len_2 + 1) {
                if char_vec_1[i - 1] == char_vec_2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    // text1のi-1文字目を削除するかtext2のj-1文字目を削除するか
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        dp[len_1][len_2]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (l1, l2) = (text1.len(), text2.len());

        let mut matrix = vec![vec![0; l2 + 1]; l1 + 1];

        for i in (0..l1).rev() {
            for j in (0..l2).rev() {
                matrix[i][j] = if text1[i] == text2[j] {
                    1 + matrix[i + 1][j + 1]
                } else {
                    matrix[i][j + 1].max(matrix[i + 1][j])
                };
            }
        }

        matrix[0][0]
    }
}

fn main() {
    let case_1 = ("abcde".to_string(), "ace".to_string());
    // 3
    let case_2 = ("abc".to_string(), "abc".to_string());
    // 3
    let case_3 = ("abc".to_string(), "def".to_string());
    // 0

    println!(
        "case_1: {}",
        Solution::longest_common_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::longest_common_subsequence(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::longest_common_subsequence(case_3.0.clone(), case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::longest_common_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::longest_common_subsequence(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::longest_common_subsequence(case_3.0.clone(), case_3.1.clone())
    );
}
