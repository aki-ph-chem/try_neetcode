// 解けなかった
struct Solution {}
impl Solution {
    // in place
    pub fn set_zeros(matrix: &mut Vec<Vec<i32>>) {}

    // in placeでない
    pub fn set_zeros_b(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = matrix.clone(); 

        result
    }

}

// C++模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn set_zeros(matrix: &mut Vec<Vec<i32>>) {
        let (column, row) = (matrix.len(), matrix[0].len());
        let (mut is_1st_row_zero, mut is_1st_col_zero) = (false, false);

        // 左端に0があるか否かの探索
        for i in 0..column {
            if matrix[i][0] == 0 {
                is_1st_col_zero = true;
                break;
            }
        }

        // 上端に0があるか否かの探索
        for j in 0..row {
            if matrix[0][j] == 0 {
                is_1st_row_zero = true;
                break;
            }
        }

        // 左端、上端以外で0である要素があるならば
        // その要素からみた左端、上端を0とする
        for i in 1..column {
            for j in 1..row {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 左端、上端が0ならば、
        // その要素から伸ばした垂線が交わる点を0とする
        for i in 1..column {
            for j in 1..row {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // 左端が0ならそこから伸ばした垂線上の要素はすべてゼロ
        if is_1st_col_zero {
            for i in 0..column {
                matrix[i][0] = 0;
            }
        }

        // 上端が0ならそこから伸ばした垂線上の要素はすべてゼロ
        if is_1st_row_zero {
            for j in 0..row {
                matrix[0][j] = 0;
            }
        }
    }
}

fn main() {
    let case_1: Vec<Vec<i32>> = vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]
        .iter()
        .map(|x| x.to_vec())
        .collect();

    let case_2: Vec<Vec<i32>> = vec![[0,1,2,0],[3,4,5,2],[1,3,1,5]]
        .iter()
        .map(|x| x.to_vec())
        .collect();

    let mut result_case_1 = case_1.clone();
    SolutionAns::set_zeros(&mut result_case_1);
    println!("case_1: {:#?}", result_case_1);

    let mut result_case_2 = case_2.clone();
    SolutionAns::set_zeros(&mut result_case_2);
    println!("case_2: {:#?}", result_case_2);
}
