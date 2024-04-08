use std::collections::VecDeque;

// お絵描きソフトの塗りつぶし機能で使われているらしい
// DFSによる実装
struct FloddFillDfs {}
impl FloddFillDfs {
    pub fn flood_fill(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, new_color: i32) {
        let prev_color = grid[x as usize][y as usize];
        if prev_color == new_color {
            return;
        }

        Self::dfs(grid, x, y, prev_color, new_color);
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, prev_color: i32, new_color: i32) {
        if x < 0
            || y < 0
            || x >= grid.len() as i32
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == new_color
            || grid[x as usize][y as usize] != prev_color
        {
            return;
        }

        grid[x as usize][y as usize] = new_color;

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in directions {
            Self::dfs(grid, x + dx, y + dy, prev_color, new_color);
        }
    }
}

// BFSによる実装
struct FloddFillBfs {}
impl FloddFillBfs {
    pub fn flood_fill(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, new_color: i32) {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        let mut seen = vec![vec![false; n]; m];

        q.push_back((x, y));
        seen[x as usize][y as usize] = true;

        while let Some(q_front) = q.pop_front() {
            let (a, b) = q_front;
            let prev_color = grid[a as usize][b as usize];
            grid[a as usize][b as usize] = new_color;

            let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (dx, dy) in directions {
                let (x_new, y_new) = (a + dx, b + dy);

                if x_new < 0
                    || y_new < 0
                    || x_new >= m as i32
                    || y_new >= n as i32
                    || seen[x_new as usize][y_new as usize]
                {
                    continue;
                }

                if grid[x_new as usize][y_new as usize] == prev_color {
                    q.push_back((x_new, y_new));
                    seen[x_new as usize][y_new as usize] = true;
                }
            }
        }
    }
}

fn show_result(grid: &Vec<Vec<i32>>) {
    for v in grid {
        for w in v {
            print!("{w} ");
        }
        println!("");
    }
}

fn main() {
    let mut screen = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 1, 0, 1, 1],
        vec![1, 2, 2, 2, 2, 0, 1, 0],
        vec![1, 1, 1, 2, 2, 0, 1, 0],
        vec![1, 1, 1, 2, 2, 2, 2, 0],
        vec![1, 1, 1, 1, 1, 2, 1, 1],
        vec![1, 1, 1, 1, 1, 2, 2, 1],
    ];

    //(4,4) にある2をprev_colorとし,new_clorを3とする
    FloddFillDfs::flood_fill(&mut screen, 4, 4, 3);
    println!("DFS");
    show_result(&screen);

    let mut screen = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 1, 0, 1, 1],
        vec![1, 2, 2, 2, 2, 0, 1, 0],
        vec![1, 1, 1, 2, 2, 0, 1, 0],
        vec![1, 1, 1, 2, 2, 2, 2, 0],
        vec![1, 1, 1, 1, 1, 2, 1, 1],
        vec![1, 1, 1, 1, 1, 2, 2, 1],
    ];

    //(4,4) にある2をprev_colorとし,new_clorを3とする
    FloddFillBfs::flood_fill(&mut screen, 4, 4, 3);
    println!("BFS");
    show_result(&screen);
}
