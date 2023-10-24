struct Solution {}
impl Solution {
    // O(log(nm) + m)
    // AC
    pub fn search_matrix_1d(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut vec_1d = vec![];
        for v in matrix {
            vec_1d.extend(v);
        }
        eprintln!("vec_1d: {:?}", vec_1d);
        let (mut left, mut right) = (0, vec_1d.len() as i32 - 1);

        while left <= right {
            let mid = (left + right) / 2;
            if target == vec_1d[mid as usize] {
                return true;
            }

            if vec_1d[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }

    // O(log(n) + log(m)) = O(log(nm))
    // AC
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut left_y, mut right_y) = (0, matrix.len() as i32 - 1);
        let mut mid_y = (left_y + right_y) / 2;
        while left_y <= right_y {
            if matrix[mid_y as usize][0] == target {
                return true;
            }

            if matrix[mid_y as usize][0] < target {
                left_y = mid_y + 1;
            } else {
                right_y = mid_y - 1;
            }
            mid_y = (left_y + right_y) / 2;
        }

        let (mut left_x, mut right_x) = (0, matrix[mid_y as usize].len() as i32 - 1);
        let mut mid_x = (left_x + right_x) / 2;
        while left_x <= right_x {
            if matrix[mid_y as usize][mid_x as usize] == target {
                return true;
            }

            if matrix[mid_y as usize][mid_x as usize] < target {
                left_x = mid_x + 1;
            } else {
                right_x = mid_x - 1;
            }
            mid_x = (left_x + right_x) / 2;
        }

        false
    }
}

// 模範解答
use std::cmp::Ordering::*;
struct SolutionAns {}
impl SolutionAns {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut t, mut b) = (0, matrix.len());
        let mut row = 0;
        while t < b {
            row = t + (b - t) / 2;
            let first = matrix[row][0];
            let last = *matrix[row].last().unwrap();
            if target == first || target == last {
                return true;
            } else if target < first {
                b = row;
            } else if target > last {
                t = row + 1;
            } else {
                break;
            }
        }

        if t > b {
            return false;
        }

        let (mut l, mut r) = (0, matrix[row].len());
        while l < r {
            let col = l + (r - l) / 2;
            match target.cmp(&matrix[row][col]) {
                Equal => return true,
                Less => r = col,
                Greater => l = col + 1,
            }
        }

        false
    }
}

fn main() {
    let case_1 = (
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3,
    );
    let case_2 = (
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13,
    );

    println!(
        "case_1: {}",
        Solution::search_matrix_1d(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::search_matrix_1d(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        Solution::search_matrix(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::search_matrix(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::search_matrix(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::search_matrix(case_2.0.clone(), case_2.1)
    );
}
