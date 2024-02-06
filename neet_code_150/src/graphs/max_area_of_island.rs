// AC
struct Solution {}
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut result, mut result_prev) = (0, 0);

        for x in 0..m {
            for y in 0..n {
                if grid[x][y] == 1 {
                    result_prev = result;
                    result = 0;
                    Self::dfs(&mut grid, (x as i32, y as i32), &mut result);
                }
                result = result.max(result_prev);
            }
        }

        result
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, v: (i32, i32), result: &mut i32) {
        let (m, n) = (grid.len(), grid[0].len());

        // 境界チェック
        if v.0 < 0
            || v.1 < 0
            || v.0 >= m as i32
            || v.1 >= n as i32
            || grid[v.0 as usize][v.1 as usize] == 0
        {
            return;
        }

        // 面積をインクリメント
        *result += 1;

        // 訪問済み
        grid[v.0 as usize][v.1 as usize] = 0;
        // 十字方向の再帰的な探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            let v_new = (v.0 + s.0, v.1 + s.1);
            Self::dfs(grid, v_new, result);
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut new_grid = grid.clone();

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if new_grid[x][y] == 1 {
                    max_area = max_area.max(Self::dfs(&mut new_grid, x as i32, y as i32));
                }
            }
        }

        max_area
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == 0
        {
            return 0;
        }

        grid[x as usize][y as usize] = 0;

        let mut count = 1;
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (add_x, add_y) in directions {
            count += Self::dfs(grid, x + add_x, y + add_y);
        }

        count
    }
}

fn main() {
    let case_1 = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    // => 6

    let case_2 = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
    // => 0

    println!("case_1: {}", Solution::max_area_of_island(case_1.clone()));
    println!("case_2: {}", Solution::max_area_of_island(case_2.clone()));

    println!(
        "case_1: {}",
        SolutionAns::max_area_of_island(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::max_area_of_island(case_2.clone())
    );
}
