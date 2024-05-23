struct Solution;
impl Solution {
    // AC
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut left, mut right) = (0, n - 1);
        let (mut top, mut bottom) = (0, n - 1);

        let mut idx = 1;
        while top <= bottom && left <= right {
            for i in left..=right {
                result[top as usize][i as usize] = idx;
                idx += 1;
            }
            top += 1;

            for i in top..=bottom {
                result[i as usize][right as usize] = idx;
                idx += 1;
            }
            right -= 1;

            if top <= bottom {
                for i in (left..=right).rev() {
                    result[bottom as usize][i as usize] = idx;
                    idx += 1;
                }
            }
            bottom -= 1;

            if left <= right {
                for i in (top..=bottom).rev() {
                    result[i as usize][left as usize] = idx;
                    idx += 1;
                }
            }
            left += 1;
        }

        result
    }
}

// Kotlinの模範解答より
// 自分の回答とほぼ同じ
struct SolutionAnsKotlin;
impl SolutionAnsKotlin {
    // AC
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut left, mut right) = (0, n - 1);
        let (mut top, mut bottom) = (0, n - 1);
        let mut count = 1;

        while left <= right && top <= bottom {
            for i in left..=right {
                result[top as usize][i as usize] = count;
                count += 1;
            }
            top += 1;

            for i in top..=bottom {
                result[i as usize][right as usize] = count;
                count += 1;
            }
            right -= 1;

            if left > right || top > bottom {
                break;
            }

            for i in (left..=right).rev() {
                result[bottom as usize][i as usize] = count;
                count += 1;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                result[i as usize][left as usize] = count;
                count += 1;
            }
            left += 1;
        }

        result
    }
}

fn main() {
    let case_1 = 3;
    // [[1,2,3],[8,9,4],[7,6,5]]
    let case_2 = 1;
    // [[1]]

    println!("case_1: {:?}", Solution::generate_matrix(case_1));
    println!("case_2: {:?}", Solution::generate_matrix(case_2));

    println!("case_1: {:?}", SolutionAnsKotlin::generate_matrix(case_1));
    println!("case_2: {:?}", SolutionAnsKotlin::generate_matrix(case_2));
}
