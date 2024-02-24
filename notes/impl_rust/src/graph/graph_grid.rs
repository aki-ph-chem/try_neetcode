use std::collections::VecDeque;
type Grid<T> = Vec<Vec<T>>;

struct DfsGridCross {}
impl DfsGridCross {
    fn search(grid: &Grid<i32>) {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                Self::dfs(&grid, &mut seen, (i as i32, j as i32));
            }
        }
        println!("end");
    }

    fn dfs(grid: &Grid<i32>, seen: &mut Grid<bool>, v: (i32, i32)) {
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

        // vを訪問済みにする
        seen[v.0 as usize][v.1 as usize] = true;

        print!("{} -> ", grid[v.0 as usize][v.1 as usize]);

        // 十字方向を再帰的に探索する
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            let v_new = (s.0 + v.0, s.1 + v.1);
            Self::dfs(grid, seen, v_new);
        }
    }
}

struct BfsGridCross {}
impl BfsGridCross {
    fn search(grid: &Grid<i32>, v: (i32, i32)) {
        let (m, n) = (grid.len(), grid[0].len());
        // 探索状況を記録
        let mut seen = vec![vec![false; n]; m];
        // 探索用のキュー
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();

        // スタート地点を訪問済みにする
        seen[v.0 as usize][v.1 as usize] = true;
        q.push_back(v);

        while !q.is_empty() {
            let (row, col) = (q.front().unwrap().0, q.front().unwrap().1);
            q.pop_front();

            print!("{} -> ", grid[row as usize][col as usize]);

            // 十字方向の探索
            let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for s in directions {
                let (x, y) = (row + s.0, col + s.1);

                // 境界チェック
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 || seen[x as usize][y as usize]
                {
                    continue;
                }

                // 訪問済みにする
                seen[x as usize][y as usize] = true;
                q.push_back((x, y));
            }
        }

        println!("end");
    }

    fn search_2(grid: &Grid<i32>, v: (i32, i32)) {
        let (m, n) = (grid.len(), grid[0].len());
        // 探索状況を記録
        let mut seen = vec![vec![false; n]; m];
        // 探索用のキュー
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();

        // スタート地点を訪問済みにする
        seen[v.0 as usize][v.1 as usize] = true;
        q.push_back(v);

        while let Some(q_front) = q.front() {
            let (row, col) = (q_front.0, q_front.1);
            q.pop_front();

            print!("{} -> ", grid[row as usize][col as usize]);

            // 十字方向の探索
            let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for s in directions {
                let (x, y) = (row + s.0, col + s.1);

                // 境界チェック
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 || seen[x as usize][y as usize]
                {
                    continue;
                }

                // 訪問済みにする
                seen[x as usize][y as usize] = true;
                q.push_back((x, y));
            }
        }

        println!("end");
    }
}

fn main() {
    let grid_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!("DFS:");
    DfsGridCross::search(&grid_1);

    println!("BFS:");
    BfsGridCross::search(&grid_1, (0, 0));
    BfsGridCross::search_2(&grid_1, (0, 0));
}
