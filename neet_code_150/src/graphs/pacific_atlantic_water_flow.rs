// 解けなかった(というよりも全く手が出なかった...)
struct Solution {}
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        result
    }

    fn dfs(heights: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>) {}
}

// AC
// C++の模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;

        let mut seen_pacific = vec![vec![false; n as usize]; m as usize];
        let mut seen_atlantic = vec![vec![false; n as usize]; m as usize];

        // 左端、右端、上端、下端からそれぞれ山を再帰的に登る
        //
        for i in 0..m {
            // (i, 0)から再帰的に探索
            // 左端からスタート
            Self::dfs(&heights, &mut seen_pacific, i, 0, m, n);
            // (i, n - 1)から再帰的に探索
            // 右端からスタート
            Self::dfs(&heights, &mut seen_atlantic, i, n - 1, m, n);
        }

        for j in 0..n {
            // (0, j)から再帰的に探索
            // 上端からスタート
            Self::dfs(&heights, &mut seen_pacific, 0, j, m, n);
            // (m - 1, j)から再帰的に探索
            // 下端からスタート
            Self::dfs(&heights, &mut seen_atlantic, m - 1, j, m, n);
        }

        let for_debug = || {
            println!("seen_pacific:");
            for arr in &seen_pacific {
                for a in arr {
                    print!("{} ", *a as i32);
                }
                println!();
            }

            println!("seen_atlantic:");
            for arr in &seen_atlantic {
                for a in arr {
                    print!("{} ", *a as i32);
                }
                println!();
            }
        };
        for_debug();

        let mut result = vec![];
        for i in 0..m {
            for j in 0..n {
                // pacific, atlantic共に訪問済みならばok
                if seen_pacific[i as usize][j as usize] && seen_atlantic[i as usize][j as usize] {
                    result.push(vec![i, j]);
                }
            }
        }

        result
    }

    fn dfs(heights: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, i: i32, j: i32, m: i32, n: i32) {
        // (i, j)を訪問済みにする
        seen[i as usize][j as usize] = true;

        // 十字方向に再帰的に探索
        //
        // (i, j) -> (i - 1, j)
        if i > 0
            && !seen[i as usize - 1][j as usize]
            && heights[i as usize - 1][j as usize] >= heights[i as usize][j as usize]
        {
            Self::dfs(heights, seen, i - 1, j, m, n);
        }

        // (i, j) -> (i + 1, j)
        if i < m - 1
            && !seen[i as usize + 1][j as usize]
            && heights[i as usize + 1][j as usize] >= heights[i as usize][j as usize]
        {
            Self::dfs(heights, seen, i + 1, j, m, n);
        }

        // (i, j) -> (i ,j - 1)
        if j > 0
            && !seen[i as usize][j as usize - 1]
            && heights[i as usize][j as usize - 1] >= heights[i as usize][j as usize]
        {
            Self::dfs(heights, seen, i, j - 1, m, n);
        }

        // (i, j) -> (i, j + 1)
        if j < n - 1
            && !seen[i as usize][j as usize + 1]
            && heights[i as usize][j as usize + 1] >= heights[i as usize][j as usize]
        {
            Self::dfs(heights, seen, i, j + 1, m, n);
        }
    }
}

fn main() {
    let case_1 = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    //=> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

    let res_1 = SolutionAns::pacific_atlantic(case_1.clone());
    println!("case_1: {:?}", res_1);
}
