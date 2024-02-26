use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn swim_in_water_bfs(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; n]; m];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut t = 0;

        // (0,0)からスタート
        seen[0][0] = true;
        q.push_back((0, 0));

        while let Some(q_front) = q.pop_front() {
            let (row, col) = (q_front.0, q_front.1);

            // 十字方向の探索
            let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for s in directions {
                let (x, y) = (row + s.0, col + s.1);

                // 境界チェック
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 || seen[x as usize][y as usize]
                {
                    continue;
                }

                if t == grid[row as usize][col as usize].max(grid[x as usize][y as usize]) {
                    seen[x as usize][y as usize] = true;
                    q.push_back((x, y));
                } else {
                    t += 1;
                }
            }
        }

        t
    }
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; n]; m];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut t = 0;

        for i in 0..m {
            for j in 0..n {
                Self::dfs(&grid, &mut seen, (i as i32, j as i32), &mut t);
            }
        }

        t
    }

    fn dfs(grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, v: (i32, i32), t: &mut i32) {
        let (m, n) = (grid.len(), grid[0].len());

        // 境界チェック
        if v.0 < 0
            || v.1 < 0
            || v.0 >= m as i32
            || v.1 >= n as i32
            || seen[v.0 as usize][v.1 as usize]
        {
            return;
        }

        // 十字方向を再帰的に探索する
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            let (x, y) = (s.0 + v.0, s.1 + v.1);
            while *t != grid[v.0 as usize][v.1 as usize].max(grid[x as usize][y as usize]) {
                *t += 1;
            }
            // vを訪問済みにする
            seen[v.0 as usize][v.1 as usize] = true;

            Self::dfs(grid, seen, (x, y), t);
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 1 {
            return 0;
        }

        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;

        let mut result = grid[0][0].max(grid[n - 1][n - 1]);
        let mut pq: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        pq.push(Reverse((result, 0, 0)));

        while let Some(pq_top) = pq.pop() {
            let current = pq_top.0;

            result = result.max(current.0);
            let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for d in directions {
                let (x, y) = (current.1 + d.0, current.2 + d.1);

                if x < 0
                    || x >= n as i32
                    || y < 0
                    || y >= n as i32
                    || visited[x as usize][y as usize]
                {
                    continue;
                }

                if x == n as i32 - 1 && y == n as i32 - 1 {
                    return result;
                }

                pq.push(Reverse((grid[x as usize][y as usize], x, y)));
                visited[x as usize][y as usize] = true;
            }
        }

        -1
    }
}

// 模範解答
mod solution_ans {
    use super::*;

    #[derive(Eq, PartialEq)]
    pub struct State {
        t: i32,
        i: usize,
        j: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.t.cmp(&self.t)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub struct SolutionAns {}
    impl SolutionAns {
        pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
            let n = grid.len();
            let mut visited = vec![vec![false; n]; n];
            let mut heap = BinaryHeap::new();
            heap.push(State {
                t: grid[0][0],
                i: 0,
                j: 0,
            });

            let mut min_t = 0;
            while let Some(State { t, i, j }) = heap.pop() {
                min_t = min_t.max(t);
                if i == n - 1 && j == n - 1 {
                    return min_t;
                }

                for (d_i, d_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                    let (n_i, n_j) = (i as i32 + d_i, j as i32 + d_j);

                    if n_i >= 0
                        && n_i < n as i32
                        && n_j >= 0
                        && n_j < n as i32
                        && !visited[n_i as usize][n_j as usize]
                    {
                        heap.push(State {
                            t: grid[n_i as usize][n_j as usize],
                            i: n_i as usize,
                            j: n_j as usize,
                        });
                        visited[n_i as usize][n_j as usize] = true;
                    }
                }
            }

            min_t
        }
    }
}

fn main() {
    let case_1 = vec![vec![0, 2], vec![1, 3]];
    // => 3
    let case_2 = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    // => 16

    println!("case_1: {}", SolutionAnsCpp::swim_in_water(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::swim_in_water(case_2.clone()));

    println!(
        "case_1: {}",
        solution_ans::SolutionAns::swim_in_water(case_1.clone())
    );
    println!(
        "case_2: {}",
        solution_ans::SolutionAns::swim_in_water(case_2.clone())
    );
}
