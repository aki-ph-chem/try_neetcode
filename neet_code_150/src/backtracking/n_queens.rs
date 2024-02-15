use std::collections::HashSet;
// 解けなかった
struct Solution {}
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec![".".to_string(); n as usize]; n as usize];

        for x in 0..n {
            for y in 0..n {
                Self::dfs(&mut board, x, y, n);
            }
        }

        board
    }

    fn dfs(board: &mut Vec<Vec<String>>, i: i32, j: i32, n: i32) {
        if i < 0 || i >= n || j < 0 || j >= n || board[i as usize][j as usize] == "Q".to_string() {
            return;
        }

        board[i as usize][j as usize] = "Q".to_string();

        // 8方位を再帰的に探索
        let directions: [(i32, i32); 8] = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        for (dx, dy) in directions {
            Self::dfs(board, i + dx, j + dy, n);
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut chessboard = vec![vec![0; n as usize]; n as usize];
        let mut result = Vec::new();
        let directions = vec![(-1, 1), (1, 1), (1, -1), (-1, -1)];
        Self::backtrack(&mut chessboard, &mut result, 0, &directions);

        result
    }

    fn backtrack(
        chessboard: &mut Vec<Vec<i8>>,
        result: &mut Vec<Vec<String>>,
        row: usize,
        directions: &Vec<(isize, isize)>,
    ) {
        if row == chessboard.len() {
            result.push(Self::copy_board(&chessboard));
            return;
        }

        for col in 0..chessboard.len() {
            if chessboard[row][col] == 0 {
                Self::toggle_queen(row, col, chessboard, directions, 1);
                Self::backtrack(chessboard, result, row + 1, directions);
                Self::toggle_queen(row, col, chessboard, directions, -1);
            }
        }

        return;
    }

    fn toggle_queen(
        row: usize,
        col: usize,
        chessboard: &mut Vec<Vec<i8>>,
        directions: &Vec<(isize, isize)>,
        toggle: i8,
    ) {
        for i in 0..chessboard.len() {
            chessboard[row][i] += toggle;
        }
        for j in 0..chessboard.len() {
            chessboard[j][col] += toggle;
        }

        for dir in directions {
            let mut i = row as isize + dir.0;
            let mut j = col as isize + dir.1;

            while i >= 0 && i < chessboard.len() as isize && j >= 0 && j < chessboard.len() as isize
            {
                chessboard[i as usize][j as usize] += toggle;
                i += dir.0;
                j += dir.1;
            }
        }

        chessboard[row][col] -= 3 * toggle;
    }

    fn copy_board(chessboard: &Vec<Vec<i8>>) -> Vec<String> {
        let mut copy = vec!['.'; chessboard.len()];
        let mut boad: Vec<String> = Vec::with_capacity(chessboard.len());

        for i in 0..chessboard.len() {
            for j in 0..chessboard.len() {
                if chessboard[i][j] == -1 {
                    copy[j] = 'Q';
                } else {
                    copy[j] = '.';
                }
            }
            boad.push(copy.iter().collect());
        }

        boad
    }
}

// C++の模範解答
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut result = Vec::new();

        let mut cols: HashSet<i32> = HashSet::new();
        let mut neg_diag: HashSet<i32> = HashSet::new();
        let mut pos_diag: HashSet<i32> = HashSet::new();

        Self::backtrack(
            n,
            0,
            &mut result,
            &mut board,
            &mut cols,
            &mut neg_diag,
            &mut pos_diag,
        );

        result
    }

    fn backtrack(
        n: i32,
        row: i32,
        res: &mut Vec<Vec<String>>,
        board: &mut Vec<Vec<char>>,
        cols: &mut HashSet<i32>,
        neg_diag: &mut HashSet<i32>,
        pos_diag: &mut HashSet<i32>,
    ) {
        if row == n {
            res.push(board.clone().iter().map(|x| x.iter().collect()).collect());
            return;
        }

        for col in 0..n {
            if cols.contains(&col)
                || neg_diag.contains(&(row - col))
                || pos_diag.contains(&(row + col))
            {
                continue;
            }

            cols.insert(col);
            neg_diag.insert(row - col);
            pos_diag.insert(row + col);
            board[row as usize][col as usize] = 'Q';

            Self::backtrack(n, row + 1, res, board, cols, neg_diag, pos_diag);

            cols.take(&col);
            neg_diag.take(&(row - col));
            pos_diag.take(&(row + col));
            board[row as usize][col as usize] = '.';
        }
    }
}

fn main() {
    let case_1 = 4;
    // => [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
    let case_2 = 1;
    // => [["Q"]]

    //println!("case_1: {:?}", Solution::solve_n_queens(case_1));

    println!("case_1: {:?}", SolutionAns::solve_n_queens(case_1));
    println!("case_2: {:?}", SolutionAns::solve_n_queens(case_2));

    println!("case_1: {:?}", SolutionAnsCpp::solve_n_queens(case_1));
    println!("case_2: {:?}", SolutionAnsCpp::solve_n_queens(case_2));
}
