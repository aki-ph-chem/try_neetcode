// 解けなかった
struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        Self::dfs(&grid, (0, 0), &mut result);

        result
    }

    fn dfs(grid: &Vec<Vec<char>>, v: (usize, usize), result: &mut i32) {
        let n = grid[0].len();
        let m = grid.len();

        let mut seen = vec![vec![false; n]; m];
        let mut todo: Vec<(usize, usize)> = vec![];

        seen[v.0][v.1] = true;
        todo.push((v.0, v.1));

        while !todo.is_empty() {
            let v = todo.pop().unwrap();

            // vから辿れるgridを探索
            for i in v.0..m {
                for j in v.1..n {
                    if seen[i][j] {
                        continue;
                    }

                    seen[i][j] = true;
                    todo.push((i, j));
                }
            }
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut new_grid = grid.clone();

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if new_grid[x][y] == '1' {
                    count += 1;
                    Self::dfs(&mut new_grid, x as i32, y as i32);
                }
            }
        }

        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, x: i32, y: i32) {
        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == '0'
        {
            return;
        }

        // 訪れたグリッドは'0'に書き換える
        grid[x as usize][y as usize] = '0';

        // 十字方向を再帰的に探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (add_x, add_y) in directions {
            Self::dfs(grid, x + add_x, y + add_y);
        }
    }
}

// C++の模範解答より
struct SolutionCppAns {}
impl SolutionCppAns {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len() as i32;
        let m = grid.len() as i32;
        let mut new_grid = grid.clone();
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if new_grid[i as usize][j as usize] == '1' {
                    result += 1;
                    Self::dfs(&mut new_grid, i, j, m, n);
                }
            }
        }

        result
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) {
        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == '0' {
            return;
        }

        // 訪れた点を'0'に書き換える
        grid[i as usize][j as usize] = '0';
        // 十字方向を再帰的に探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (add_x, add_y) in directions {
            Self::dfs(grid, i + add_x, j + add_y, m, n);
        }
    }
}

fn main() {
    let case_1 = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    // => 1
    let case_2 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    // => 3

    /*
    println!("case_1: {}", Solution::num_islands(case_1.clone()));
    println!("case_2: {}", Solution::num_islands(case_2.clone()));
        */

    println!("case_1: {}", SolutionAns::num_islands(case_1.clone()));
    println!("case_2: {}", SolutionAns::num_islands(case_2.clone()));

    println!("case_1: {}", SolutionCppAns::num_islands(case_1.clone()));
    println!("case_2: {}", SolutionCppAns::num_islands(case_2.clone()));
}
