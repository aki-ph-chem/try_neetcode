// Rustの模範解答はないので
// C++のものを参考に実装
struct SolutionAns {}
impl SolutionAns {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let (mut left, mut right) = (0, matrix[0].len() as i32 - 1);
        let (mut top, mut bottom) = (0, matrix.len() as i32 - 1);

        while top <= bottom && left <= right {
            for i in left..right + 1 {
                result.push(matrix[top as usize][i as usize]);
            }
            top += 1;

            for i in top..bottom + 1 {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for i in (left..right + 1).rev() {
                    result.push(matrix[bottom as usize][i as usize]);
                }
            }
            bottom -= 1;

            if left <= right {
                for i in (top..bottom + 1).rev() {
                    result.push(matrix[i as usize][left as usize]);
                }
            }
            left += 1;
        }

        result
    }
}

fn main() {
    let case_1: Vec<Vec<i32>> = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let case_2: Vec<Vec<i32>> = vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
        .iter()
        .map(|x| x.to_vec())
        .collect();

    println!("case_1: {:?}", SolutionAns::spiral_order(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::spiral_order(case_2.clone()));
}
