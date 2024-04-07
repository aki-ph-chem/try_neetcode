use std::collections::HashSet;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut result = 0;

        for x in 0..grid.len() as i32 {
            for y in 0..grid[0].len() as i32 {
                if grid[x as usize][y as usize] == 1 {
                    Self::dfs(&mut grid, &mut result, x, y);
                }
            }
        }

        if result != 0 {
            result += 2;
        }

        result
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, sum: &mut i32, x: i32, y: i32) {
        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == 0
        {
            return;
        }

        *sum += 2;

        // 訪れたグリッドは0に書き換える
        grid[x as usize][y as usize] = 0;

        // 十字方向を再帰的に探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (d_x, d_y) in directions {
            Self::dfs(grid, sum, x + d_x, y + d_y);
        }
    }
}

// C++の模範解答より(BFSで解く)
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    result += 4;

                    if j > 0 && grid[i][j - 1] == 1 {
                        result -= 2;
                    }
                    if i > 0 && grid[i - 1][j] == 1 {
                        result -= 2;
                    }
                }
            }
        }

        result
    }
}

// Pythonの模範解答より(DFSで解く)
struct SolutionAnsPython {}
impl SolutionAnsPython {
    // AC
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    //println!("({}, {})", i, j);
                    return Self::dfs(&grid, &mut set, i as i32, j as i32);
                }
            }
        }

        0
    }

    fn dfs(grid: &Vec<Vec<i32>>, visit: &mut HashSet<(i32, i32)>, i: i32, j: i32) -> i32 {
        if i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || i < 0
            || j < 0
            || grid[i as usize][j as usize] == 0
        {
            return 1;
        }

        if visit.contains(&(i, j)) {
            return 0;
        }

        visit.insert((i, j));
        let mut result = 0;
        let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (d_x, d_y) in directions {
            result += Self::dfs(grid, visit, i + d_x, j + d_y);
        }

        result
    }
}

fn main() {
    let case_1 = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    // => 16
    let case_2 = vec![vec![1]];
    // => 4
    let case_3 = vec![vec![1, 0]];
    // => 4

    /*
    println!("case_1: {}", Solution::island_perimeter(case_1.clone()));
    println!("case_2: {}", Solution::island_perimeter(case_2.clone()));
    println!("case_3: {}", Solution::island_perimeter(case_3.clone()));
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::island_perimeter(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::island_perimeter(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::island_perimeter(case_3.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsPython::island_perimeter(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::island_perimeter(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsPython::island_perimeter(case_3.clone())
    );
}
