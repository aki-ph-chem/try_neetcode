// 解けなかった
struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // dp[i][j]: (i,j)までの経路の数
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        dp[0][0] = 1;

        for i in 1..=(m as usize) {
            for j in 1..=(n as usize) {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] += dp[i][j - 1];
                dp[i][j] += dp[i - 1][j];
            }
        }

        dp[m as usize][n as usize]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut bottom = vec![1; n as usize];

        for _i in 0..m - 1 {
            let mut top = vec![1; n as usize];
            for c in (0..n - 1).rev().map(|c| c as usize) {
                top[c] = bottom[c] + top[c + 1];
            }

            bottom = top;
        }

        bottom[0]
    }

    // AC
    // C++の模範解答より
    pub fn unique_paths_2(m: i32, n: i32) -> i32 {
        let mut grid = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            grid[i][0] = 1;
        }

        for j in 0..n as usize {
            grid[0][j] = 1;
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
            }
        }

        grid[m as usize - 1][n as usize - 1]
    }
}

fn main() {
    let case_1 = (3, 7);
    // 28
    let case_2 = (3, 2);
    // 3

    println!("case_1: {}", Solution::unique_paths(case_1.0, case_1.1));
    println!("case_2: {}", Solution::unique_paths(case_2.0, case_2.1));

    println!("case_1: {}", SolutionAns::unique_paths(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::unique_paths(case_2.0, case_2.1));

    println!("case_1: {}", SolutionAns::unique_paths_2(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::unique_paths_2(case_2.0, case_2.1));
}
