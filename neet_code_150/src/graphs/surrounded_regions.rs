// 解けなかった
struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());

        for x in 0..m {
            for y in 0..n {
                if board[x][y] == 'O' {
                    Self::dfs(board, (x as i32, y as i32));
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, v: (i32, i32)) {
        let (m, n) = (board.len(), board[0].len());
        // 境界チェック
        if v.0 < 0
            || v.1 < 0
            || v.0 >= m as i32
            || v.1 >= n as i32
            || board[v.0 as usize][v.1 as usize] == 'X'
        {
            return;
        }

        // 訪問済み
        board[v.0 as usize][v.1 as usize] = 'X';
        // 十字方向の再帰的な探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            let v_new = (v.0 + s.0, v.1 + s.1);
            Self::dfs(board, v_new);
        }
    }

    fn dfs_2(board: &mut Vec<Vec<char>>, v: (i32, i32)) -> bool {
        let (m, n) = (board.len(), board[0].len());

        // 十字方向の再帰的な探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            if board[v.0 as usize][v.1 as usize] == 'O' {
                let v_new = (v.0 + s.0, v.1 + s.1);
            }
        }

        true
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());

        // 右端、左端
        for i in 0..m {
            Self::dfs(board, (i as i32, 0));
            Self::dfs(board, (i as i32, n as i32 - 1));
        }

        // 上端、下端
        for j in 0..n {
            Self::dfs(board, (0, j as i32));
            Self::dfs(board, (m as i32 - 1, j as i32));
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == 'E' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, v: (i32, i32)) {
        let (m, n) = (board.len(), board[0].len());

        // 境界チェック
        if v.0 < 0
            || v.0 >= m as i32
            || v.1 < 0
            || v.1 >= n as i32
            || board[v.0 as usize][v.1 as usize] != 'O'
        {
            return;
        }

        // 'O'を'E'に書き換える
        board[v.0 as usize][v.1 as usize] = 'E';

        // 十字方向の再帰的な探索
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for s in directions {
            let v_new = (v.0 + s.0, v.1 + s.1);
            Self::dfs(board, v_new);
        }
    }
}

fn main() {
    let case_1 = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    // =>
    /*
       [ ['X', 'X', 'X', 'X']
         ['X', 'X', 'X', 'X']
         ['X', 'X', 'X', 'X']
         ['X', 'O', 'X', 'X']]
    */
    let case_2 = vec![vec!['X']];
    // => [['X']]

    let mut res_1 = case_1.clone();
    let mut res_2 = case_2.clone();
    SolutionAnsCpp::solve(&mut res_1);
    SolutionAnsCpp::solve(&mut res_2);

    println!("res_1: {:?}", res_1);
    println!("res_2: {:?}", res_2);
}
