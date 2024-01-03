// 解けなかった
struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let sub_word: Vec<char> = word.chars().collect();

        Self::helper(&board, &sub_word, 0, 0, 0)
    }

    // n_x,n_y: boardの添字
    // idx: sub_arrayの添字
    fn helper(
        board: &Vec<Vec<char>>,
        sub_word: &Vec<char>,
        n_x: usize,
        n_y: usize,
        mut idx: usize,
    ) -> bool {
        if sub_word.len() - 1 <= idx {
            return false;
        }

        if board[n_y][n_x] != sub_word[idx] {
            return false;
        } else {
            idx += 1;
        }

        let len_y = board.len();
        let len_x = board[0].len();
        for i in n_x..len_x {
            for j in n_y..len_y {
                if i + 1 < len_x - 1 {
                    Self::helper(board, sub_word, i + 1, j, idx);
                }

                if j + 1 < len_y - 1 {
                    Self::helper(board, sub_word, i, j + 1, idx);
                }

                if 0 < i as i32 - 1 {
                    Self::helper(board, sub_word, i - 1, j, idx);
                }

                if 0 < j as i32 - 1 {
                    Self::helper(board, sub_word, i, j - 1, idx);
                }
            }
        }

        true
    }
}

// C++の模範解答より
// AC
struct SolutionAns {}
impl SolutionAns {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let vec_char: Vec<char> = word.chars().collect();
        let n = board[0].len() as i32;
        let m = board.len() as i32;

        for i in 0..m {
            for j in 0..n {
                if board[i as usize][j as usize] == vec_char[0] {
                    if Self::dfs(&mut board, &vec_char, 0, i, j, m, n) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        vec_char: &Vec<char>,
        idx: usize,
        i: i32,
        j: i32,
        m: i32,
        n: i32,
    ) -> bool {
        if i < 0 || i >= m || j < 0 || j >= n || board[i as usize][j as usize] != vec_char[idx] {
            return false;
        }

        if idx == vec_char.len() - 1 {
            return true;
        }

        // 一旦変更
        board[i as usize][j as usize] = '#';

        if Self::dfs(board, vec_char, idx + 1, i - 1, j, m, n)
            || Self::dfs(board, vec_char, idx + 1, i, j - 1, m, n)
            || Self::dfs(board, vec_char, idx + 1, i + 1, j, m, n)
            || Self::dfs(board, vec_char, idx + 1, i, j + 1, m, n)
        {
            return true;
        }

        // 元に戻す
        board[i as usize][j as usize] = vec_char[idx];

        false
    }
}

fn main() {
    let case_1 = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        "ABCCED".to_string(),
    );
    // => true
    let case_2 = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        "SEE".to_string(),
    );
    // => true
    let case_3 = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        "ABCD".to_string(),
    );
    // => false

    /*
    println!(
        "case_1: {}",
        Solution::exist(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::exist(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::exist(case_3.0.clone(), case_3.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::exist(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::exist(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::exist(case_3.0.clone(), case_3.1.clone())
    );
}
