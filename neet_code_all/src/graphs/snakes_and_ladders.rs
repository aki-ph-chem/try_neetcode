use std::collections::{HashSet, VecDeque};

// さっぱりわからなかった...
struct Solution {}
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        1
    }
}

// Pythonの模範解答より
struct SolutionAnsPython {}
impl SolutionAnsPython {
    // AC
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut board = board;
        board.reverse();

        let mut q = VecDeque::new();
        q.push_back((1, 0));
        let mut visit = HashSet::new();

        // BFS
        while let Some(q_top) = q.pop_front() {
            let (square, moves) = (q_top.0, q_top.1);
            for i in 1..7 {
                let mut nex_square = square + i;
                let (x, y) = Self::square_to_position(nex_square, n as i32);

                if board[x as usize][y as usize] != -1 {
                    nex_square = board[x as usize][y as usize];
                }
                if nex_square == (n as i32).pow(2) {
                    return moves + 1;
                }
                if !visit.contains(&nex_square) {
                    visit.insert(nex_square);
                    q.push_back((nex_square, moves + 1));
                }
            }
        }

        -1
    }

    fn square_to_position(square: i32, n: i32) -> (i32, i32) {
        let x = (square - 1) / n;
        let mut y = (square - 1) % n;
        if x % 2 != 0 {
            y = n - 1 - y;
        }

        (x, y)
    }
}

fn main() {
    let case_1 = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    // => 4
    let case_2 = vec![vec![-1, -1], vec![-1, 3]];
    // => 1

    println!(
        "case_1: {}",
        SolutionAnsPython::snakes_and_ladders(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::snakes_and_ladders(case_2.clone())
    );
}
