struct Solution;
impl Solution {
    // AC(JSの模範解答と全く同じだった)
    // (i,i) と (i, n - 1 - i)が等しい場合は除くことでダブルカウントを防ぐ
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let n = mat.len();
        for i in 0..n {
            result += mat[i][i];
            if i != n - 1 - i {
                result += mat[i][n - 1 - i];
            }
        }

        result
    }
}

// Kotlinの模範解答より
struct SolutionAnsKotlin;
impl SolutionAnsKotlin {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len() - 1;
        let mut result = 0;
        for i in 0..=n {
            result += mat[i][i] + mat[n - i][i];
        }

        // nが偶数のときは(n/2, n/2)をダブルカウントするのであとから引き算しておく
        if n % 2 == 0 {
            result -= mat[n / 2][n / 2];
        }

        result
    }
}

fn main() {
    let case_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // => 25
    let case_2 = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    // => 8
    let case_3 = vec![vec![5]];
    // =>5

    println!("case_1: {}", Solution::diagonal_sum(case_1.clone()));
    println!("case_2: {}", Solution::diagonal_sum(case_2.clone()));
    println!("case_3: {}", Solution::diagonal_sum(case_3.clone()));

    println!(
        "case_1: {}",
        SolutionAnsKotlin::diagonal_sum(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::diagonal_sum(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsKotlin::diagonal_sum(case_3.clone())
    );
}
