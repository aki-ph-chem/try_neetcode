struct Solution {}
impl Solution {
    // inplaceで実装すること
    // 出来なかった
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let matrix_size = matrix.len();

        for i in 0..matrix_size {
            for j in 0..matrix_size {
                matrix[i][matrix_size - 1 - j] = matrix[j][i];
            }
        }
    }

    // inplaceではないとりあえずの実装
    pub fn rotate_b(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let matrix_size = matrix.len();
        let mut result = vec![vec![0; matrix_size]; matrix_size];

        for j in 0..matrix_size {
            for i in 0..matrix_size {
                result[i][matrix_size - 1 - j] = matrix[j][i];
            }
        }

        result
    }
}

struct SolutionAns {}
impl SolutionAns {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        matrix.reverse();
        //println!("marix in func: {:#?}", matrix);
        let len = matrix.len();
        for i in 0..len {
            for j in i..len {
                let x = matrix[i][j];
                let y = matrix[j][i];
                matrix[j][i] = x;
                matrix[i][j] = y;
            }
        }
    }
}

fn main() {
    let case_1: Vec<Vec<i32>> = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    // result
    /*
      [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]
      ]
    reverse()
    =>
      [
      [7, 8, 9]
      [4, 5, 6],
      [1, 2, 3],
      ]
    transpose
    =>
    [
    [7,4,1],
    [8,5,2],
    [9,6,3]
    ]
         */
    let case_2: Vec<Vec<i32>> = vec![
        [5, 1, 9, 11],
        [2, 4, 8, 10],
        [13, 3, 6, 7],
        [15, 14, 12, 16],
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect();
    // result
    /*
        [
    [15,13,2,5],
    [14,3,4,1],
    [12,6,8,9],
    [16,7,10,11]
        ]
             */

    //println!("{:#?}", Solution::rotate_b(&case_1));
    //println!("{:#?}", Solution::rotate_b(&case_2));
    let mut result_1 = case_1.clone();
    SolutionAns::rotate(&mut result_1);
    println!("{:?}", result_1);

    let mut result_2 = case_2.clone();
    SolutionAns::rotate(&mut result_2);
    println!("{:?}", result_2);
}
