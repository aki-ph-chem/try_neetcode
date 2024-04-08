use std::collections::HashSet;

struct Solution {}
impl Solution {
    // TLE
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut grid1 = grid1;
        let mut grid2 = grid2;

        let mut set_grid_1 = vec![];
        let mut set_grid_2 = vec![];

        for i in 0..grid1.len() {
            for j in 0..grid1[0].len() {
                let mut set_1 = HashSet::new();
                if grid1[i][j] == 1 {
                    Self::dfs(&mut grid1, &mut set_1, i as i32, j as i32);
                }
                if !set_1.is_empty() {
                    set_grid_1.push(set_1);
                }

                let mut set_2 = HashSet::new();
                if grid2[i][j] == 1 {
                    Self::dfs(&mut grid2, &mut set_2, i as i32, j as i32);
                }
                if !set_2.is_empty() {
                    set_grid_2.push(set_2);
                }
            }
        }

        //println!("set_grid_1: {:?}", set_grid_1);
        //println!("set_grid_2: {:?}", set_grid_2);

        let mut result = 0;

        for set_1 in &set_grid_1 {
            for set_2 in &set_grid_2 {
                let mut is_sub_set = true;

                for p in set_2 {
                    if !set_1.contains(&p) {
                        is_sub_set = false;
                        break;
                    }
                }

                if is_sub_set {
                    result += 1;
                }
            }
        }

        result
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, set: &mut HashSet<(i32, i32)>, i: i32, j: i32) {
        if i < 0
            || j < 0
            || i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == 0
        {
            return;
        }

        grid[i as usize][j as usize] = 0;

        set.insert((i, j));
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in directions {
            Self::dfs(grid, set, i + dx, j + dy);
        }
    }
}

// C++の模範解答より
type Grid = Vec<Vec<i32>>;
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid1.len(), grid1[0].len());
        let mut visit = HashSet::new();
        let mut result = 0;

        for x in 0..n {
            for y in 0..m {
                if grid2[x][y] == 1
                    && !visit.contains(&(x as i32, y as i32))
                    && Self::dfs(&grid1, &grid2, &mut visit, x as i32, y as i32)
                {
                    result += 1;
                }
            }
        }

        result
    }

    fn dfs(grid1: &Grid, grid2: &Grid, visit: &mut HashSet<(i32, i32)>, x: i32, y: i32) -> bool {
        let (n, m) = (grid1.len() as i32, grid1[0].len() as i32);

        if x < 0
            || y < 0
            || x == n
            || y == m
            || grid2[x as usize][y as usize] == 0
            || visit.contains(&(x, y))
        {
            return true;
        }

        visit.insert((x, y));
        let mut result = if grid1[x as usize][y as usize] == 0 {
            false
        } else {
            true
        };

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in directions {
            result = Self::dfs(grid1, grid2, visit, x + dx, y + dy) && result;
        }

        result
    }
}

fn main() {
    let case_1 = (
        vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ],
        vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ],
    );
    // => 3
    let case_2 = (
        vec![
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
        ],
        vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
        ],
    );
    // => 2

    /*
    println!(
        "case_1: {}",
        Solution::count_sub_islands(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::count_sub_islands(case_2.0.clone(), case_2.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::count_sub_islands(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::count_sub_islands(case_2.0.clone(), case_2.1.clone())
    );
}
