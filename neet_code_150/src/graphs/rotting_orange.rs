use std::collections::VecDeque;
// 解けなかった
// 幅優先で腐らせていけば題意を満たしそう
struct Solution {}
impl Solution {
    pub fn orange_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    Self::bfs(&mut grid, (i as i32, j as i32), &mut result);
                }
            }
        }

        result
    }

    fn bfs(grid: &mut Vec<Vec<i32>>, v: (i32, i32), result: &mut i32) {
        let (m, n) = (grid.len(), grid[0].len());

        // 十字方向の探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut is_update = false;
        for s in directions {
            let v_new = (v.0 + s.0, v.1 + s.1);
            if v_new.0 < 0
                || v_new.1 < 0
                || v_new.0 >= m as i32
                || v_new.1 >= n as i32
                || grid[v_new.0 as usize][v_new.1 as usize] != 1
            {
                continue;
            }

            grid[v_new.0 as usize][v_new.1 as usize] = 2;
            is_update = true;
        }
        if is_update {
            *result += 1;
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn orange_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        // q: 腐ったオレンジのある場所のキュー
        // fresh: 新鮮なオレンジの数
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut fresh = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    q.push_back((i as i32, j as i32));
                } else if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }

        // ダミーの点(-1, -1)
        q.push_back((-1, -1));
        let mut result = -1;

        while !q.is_empty() {
            let (row, col) = (q.front().unwrap().0, q.front().unwrap().1);
            q.pop_front();

            if row == -1 {
                result += 1;
                if !q.is_empty() {
                    q.push_back((-1, -1));
                }
            } else {
                let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for s in directions {
                    let (x, y) = (row + s.0, col + s.1);

                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }

                    if grid[x as usize][y as usize] == 1 {
                        grid[x as usize][y as usize] = 2;
                        fresh -= 1;
                        q.push_back((x, y));
                    }
                }
            }
        }

        if fresh == 0 {
            result
        } else {
            -1
        }
    }
}
fn main() {
    let case_1 = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    // => 4
    let case_2 = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    // => -1

    /*
    println!("case_1: {}", Solution::orange_rotting(case_1));
    println!("case_2: {}", Solution::orange_rotting(case_2));
    */

    println!("case_1: {}", SolutionAnsCpp::orange_rotting(case_1));
    println!("case_2: {}", SolutionAnsCpp::orange_rotting(case_2));
}
