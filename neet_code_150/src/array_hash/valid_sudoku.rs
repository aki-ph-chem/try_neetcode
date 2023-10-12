use std::collections::HashSet;

struct Solution {}
impl Solution {
    // 解けなかった😭
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut number_set = HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' {
                    number_set.insert(board[i][j]);
                }
            }
        }
        false
    }

    // C++のpassする模範解答をRustに変換した
    pub fn is_valid_sudoku_2(board: Vec<Vec<char>>) -> bool {
        const CNT:usize = 9;
        let mut row: [[bool; CNT]; CNT] = [[false; CNT]; CNT];
        let mut col: [[bool; CNT]; CNT] = [[false; CNT]; CNT];
        let mut sub: [[bool; CNT]; CNT] = [[false; CNT]; CNT];

        for r in 0..CNT {
            for c in 0..CNT {
                // 数字でないならpass
                if board[r][c] == '.' {
                    continue;
                }

                // char('1' ~ '9') -> index
                let index = board[r][c] as usize  - '0' as usize - 1;
                let area = (r / 3) * 3 + (c / 3);

                // if nubmer already exists
                if row[r][index] || col[c][index] || sub[area][index] {
                    return false;
                }

                row[r][index] = true;
                col[c][index] = true;
                sub[area][index] = true;
            }
        }
        true
    }
}

// 模範解答が間違っている!
struct SolutionAns {}
impl SolutionAns {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = HashSet::new();
        let mut col = HashSet::new();
        let mut bx = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let r = board[i][j];
                let c = board[j][i];
                let b = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];

                if r != '.' {
                    if row.insert(r) {
                        return false;
                    }
                }

                if c != '.' {
                    if col.insert(c) {
                        return false;
                    }
                }

                if b != '.' {
                    if bx.insert(b) {
                        return false;
                    }
                }
            }

            row.clear();
            col.clear();
            bx.clear();
        }
        true
    }
}

fn main() {
    let case_1 = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    //println!("case_1: {}", SolutionAns::is_valid_sudoku(case_1.clone()));
    println!("case_1: {}", Solution::is_valid_sudoku_2(case_1.clone()));

    let case_2 = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    //println!("case_2: {}", SolutionAns::is_valid_sudoku(case_2.clone()));
    println!("case_1: {}", Solution::is_valid_sudoku_2(case_2.clone()));
}
