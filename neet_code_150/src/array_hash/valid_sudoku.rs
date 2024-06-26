use std::collections::{HashMap, HashSet};

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
        const CNT: usize = 9;
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
                let index = board[r][c] as usize - '0' as usize - 1;
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
                    if !row.insert(r) {
                        return false;
                    }
                }

                if c != '.' {
                    if !col.insert(c) {
                        return false;
                    }
                }

                if b != '.' {
                    if !bx.insert(b) {
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

// AC
// Pythonの模範解答より
struct SolutionAnsPython {}
impl SolutionAnsPython {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // key = r, key = c
        let mut row: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut col: HashMap<i32, HashSet<char>> = HashMap::new();
        // key = (r/3, c/3)
        let mut subcell: HashMap<(i32, i32), HashSet<char>> = HashMap::new();

        for r in 0..9_i32 {
            for c in 0..9_i32 {
                if board[r as usize][c as usize] == '.' {
                    continue;
                }
                let b_rc = board[r as usize][c as usize];

                for opt in [row.get(&r), col.get(&c), subcell.get(&(r / 3, c / 3))] {
                    if let Some(opt_raw) = opt {
                        if opt_raw.contains(&b_rc) {
                            return false;
                        }
                    }
                }

                row.entry(r).or_insert(HashSet::from([b_rc])).insert(b_rc);
                col.entry(c).or_insert(HashSet::from([b_rc])).insert(b_rc);
                subcell
                    .entry((r / 3, c / 3))
                    .or_insert(HashSet::from([b_rc]))
                    .insert(b_rc);
            }
        }

        true
    }
}

// 後から解いたときの別解
struct SolutionLatter;
impl SolutionLatter {
    // AC
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set_h = vec![HashSet::new(); 9];
        let mut set_v = vec![HashSet::new(); 9];
        let mut set_c = vec![vec![HashSet::new(); 3]; 3];

        for a in 0..9 {
            for b in 0..9 {
                let c = board[a][b];
                if c == '.' {
                    continue;
                }

                if set_h[a].contains(&c)
                    || set_v[b].contains(&c)
                    || set_c[a / 3][b / 3].contains(&c)
                {
                    return false;
                } else {
                    set_h[a].insert(c);
                    set_v[b].insert(c);
                    set_c[a / 3][b / 3].insert(c);
                }
            }
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

    println!("case_1: {}", Solution::is_valid_sudoku_2(case_1.clone()));
    println!("case_2: {}", Solution::is_valid_sudoku_2(case_2.clone()));

    println!("case_1: {}", SolutionAns::is_valid_sudoku(case_1.clone()));
    println!("case_2: {}", SolutionAns::is_valid_sudoku(case_2.clone()));

    println!(
        "case_1: {}",
        SolutionAnsPython::is_valid_sudoku(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::is_valid_sudoku(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionLatter::is_valid_sudoku(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionLatter::is_valid_sudoku(case_2.clone())
    );
}
