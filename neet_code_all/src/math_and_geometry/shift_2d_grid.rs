struct Solution;
impl Solution {
    // AC
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut prev = grid;
        let mut result = prev.clone();

        for _iter in 0..k {
            for i in 0..m {
                for j in 0..n {
                    if j + 1 < n {
                        result[i][j + 1] = prev[i][j];
                    }
                }
            }
            for i in 0..m {
                if i + 1 < m {
                    result[i + 1][0] = prev[i][n - 1];
                }
            }
            result[0][0] = prev[m - 1][n - 1];

            prev = result.clone();
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let pos_to_val = |r, c| r * n + c;
        let val_to_pos = |v| (v / n, v % n);

        let mut result = vec![];
        for _r in 0..m {
            let mut row = vec![];
            for _c in 0..n {
                row.push(0);
            }
            result.push(row);
        }
        for r in 0..m {
            for c in 0..n {
                let new_val = (pos_to_val(r, c) + k as usize) % (m * n);
                let (idx_r, idx_c) = val_to_pos(new_val);
                result[idx_r][idx_c] = grid[r][c];
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1);
    // => [[9,1,2],[3,4,5],[6,7,8]]
    let case_2 = (
        vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ],
        4,
    );
    // => [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
    let case_3 = (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9);
    // => [[1,2,3],[4,5,6],[7,8,9]]

    println!(
        "case_1: {:?}",
        Solution::shift_grid(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::shift_grid(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        Solution::shift_grid(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::shift_grid(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::shift_grid(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::shift_grid(case_3.0.clone(), case_3.1)
    );
}
