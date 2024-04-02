// 解けなかった
struct Solution {}
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut grid = grid;
        let n = grid[0].len();

        let mut prefix_sum_0 = vec![0; n + 1];
        let mut prefix_sum_1 = vec![0; n + 1];
        prefix_sum_0[0] = 0;
        prefix_sum_1[0] = 0;

        for i in 0..n {
            prefix_sum_0[i + 1] = prefix_sum_0[i] + grid[0][i] as i64;
            prefix_sum_1[i + 1] = prefix_sum_1[i] + grid[1][i] as i64;
        }

        let mut player_1 = 0;
        let mut i_down = 0;
        for i in 0..n {
            let sum_0 = prefix_sum_0[i + 1] - prefix_sum_0[0];
            let sum_1 = prefix_sum_1[n] - prefix_sum_1[i];
            if player_1 < sum_0 + sum_1 {
                player_1 = sum_0 + sum_1;
                i_down = i;
            }
        }
        println!("player_1: {}", player_1);
        println!("i_down: {}", i_down);

        for i in 0..n {
            if i <= i_down {
                grid[0][i] = 0;
            }

            if i + i_down < n {
                grid[1][i + i_down] = 0;
            }
        }

        println!("grid[0]: {:?}", grid[0]);
        println!("grid[1]: {:?}", grid[1]);

        let mut prefix_sum_0 = vec![0; n + 1];
        let mut prefix_sum_1 = vec![0; n + 1];
        prefix_sum_0[0] = 0;
        prefix_sum_1[0] = 0;

        for i in 0..n {
            prefix_sum_0[i + 1] = prefix_sum_0[i] + grid[0][i] as i64;
            prefix_sum_1[i + 1] = prefix_sum_1[i] + grid[1][i] as i64;
        }

        let mut player_2 = 0_i64;
        for i in 0..n {
            let sum_0 = prefix_sum_0[i + 1] - prefix_sum_0[0];
            let sum_1 = prefix_sum_1[n] - prefix_sum_1[i];
            player_2 = player_2.max(sum_0 + sum_1);
        }

        player_2
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut memo_1 = vec![0; n + 1];
        let mut memo_2 = vec![0; n + 1];
        for i in 0..n {
            memo_1[i + 1] = memo_1[i] + grid[0][i] as i64;
            memo_2[i + 1] = memo_2[i] + grid[1][i] as i64;
        }

        let mut result = i64::MAX;
        for i in 0..n {
            result = result.min(memo_2[i].max(memo_1[n] - memo_1[i + 1]));
        }

        result
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let (mut top, mut bottom) = (grid[0][0] as i64, 0);
        let mut result = i64::MAX;

        for i in 1..n {
            top += grid[0][i] as i64;
        }

        for i in 0..n {
            top -= grid[0][i] as i64;

            result = result.min(top.max(bottom));

            bottom += grid[1][i] as i64;
        }

        result
    }
}

// grid[r][c]: (r,c)におけるポイント,(2 x n)

// (r, c) -> (r, c + 1)
// (r, c) -> (r + 1, c)

fn main() {
    let case_1 = vec![vec![2, 5, 4], vec![1, 5, 1]];
    // => 4
    let case_2 = vec![vec![3, 3, 1], vec![8, 5, 2]];
    // => 4
    let case_3 = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
    // => 7

    /*
    println!("case_1: {}", Solution::grid_game(case_1.clone()));
    println!("case_2: {}", Solution::grid_game(case_2.clone()));
    println!("case_3: {}", Solution::grid_game(case_3.clone()));
    */

    println!("case_1: {}", SolutionAns::grid_game(case_1.clone()));
    println!("case_2: {}", SolutionAns::grid_game(case_2.clone()));
    println!("case_3: {}", SolutionAns::grid_game(case_3.clone()));

    println!("case_1: {}", SolutionAnsCpp::grid_game(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::grid_game(case_2.clone()));
    println!("case_3: {}", SolutionAnsCpp::grid_game(case_3.clone()));
}
