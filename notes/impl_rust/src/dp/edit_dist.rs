fn edit_dist(s: &Vec<char>, t: &Vec<char>) -> i64 {
    let chmin = |b: i64, a: &mut i64| {
        if *a > b {
            *a = b;
        }
    };

    let (n_s, n_t) = (s.len(), t.len());
    let mut dp = vec![vec![std::i64::MAX; n_t + 1]; n_s + 1];
    dp[0][0] = 0;

    for i in 0..=n_s {
        for j in 0..=n_t {
            // 変更
            if i > 0 && j > 0 {
                if s[i - 1] == t[j - 1] {
                    chmin(dp[i - 1][j - 1], &mut dp[i][j]);
                } else {
                    chmin(dp[i - 1][j - 1] + 1, &mut dp[i][j]);
                }
            }

            if i > 0 {
                // 削除
                chmin(dp[i - 1][j] + 1, &mut dp[i][j]);
            }

            if j > 0 {
                // 挿入
                chmin(dp[i][j - 1] + 1, &mut dp[i][j]);
            }
        }
    }

    dp[n_s][n_t]
}

fn main() {
    let case_1 = (
        "kitten".chars().collect::<Vec<char>>(),
        "sitting".chars().collect::<Vec<char>>(),
    );
    // => 3
    let case_2 = (
        "horse".chars().collect::<Vec<char>>(),
        "ros".chars().collect::<Vec<char>>(),
    );
    // => 3

    println!("case_1: {}", edit_dist(&case_1.0, &case_1.1));
    println!("case_2: {}", edit_dist(&case_2.0, &case_2.1));
}
