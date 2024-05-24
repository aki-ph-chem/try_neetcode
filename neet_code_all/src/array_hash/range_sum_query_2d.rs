// AC
struct NumMatrix {
    prefix_sum: Vec<Vec<i32>>,
}
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut prefix_sum = vec![vec![0; n + 1]; m];

        for i_m in 0..m {
            for i_n in 0..n {
                prefix_sum[i_m][i_n + 1] = prefix_sum[i_m][i_n] + matrix[i_m][i_n];
            }
        }
        //println!("prefix_sum: {:?}", prefix_sum);

        Self { prefix_sum }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;

        for i in row1..=row2 {
            sum += self.prefix_sum[i as usize][col2 as usize + 1]
                - self.prefix_sum[i as usize][col1 as usize];
        }

        sum
    }
}

// Pythonの模範解答より
// AC
struct NumMatrixPy {
    prefix_sum: Vec<Vec<i32>>,
}
impl NumMatrixPy {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut prefix_sum = vec![vec![0; n + 1]; m + 1];

        for (i, line) in matrix.iter().enumerate() {
            let mut prev = 0;
            for (j, num) in line.iter().enumerate() {
                prev += num;
                prefix_sum[i + 1][j + 1] = prev + prefix_sum[i][j + 1];
            }
        }

        Self { prefix_sum }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let sum_col2 = self.prefix_sum[row2 as usize + 1][col2 as usize + 1]
            - self.prefix_sum[row1 as usize][col2 as usize + 1];
        let sum_col1 = self.prefix_sum[row2 as usize + 1][col1 as usize]
            - self.prefix_sum[row1 as usize][col1 as usize];

        sum_col2 - sum_col1
    }
}

// C++の模範解答
struct NumMatrixCpp {
    dp: Vec<Vec<i32>>,
}
impl NumMatrixCpp {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (r, c) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut dp = matrix;

        for i in 0..r {
            for j in 0..c {
                if i > 0 {
                    dp[i as usize][j as usize] += dp[i as usize - 1][j as usize];
                }
                if j > 0 {
                    dp[i as usize][j as usize] += dp[i as usize][j as usize - 1];
                }

                // ダブルカウントしたものを引く
                if i > 0 && j > 0 {
                    dp[i as usize][j as usize] -= dp[i as usize - 1][j as usize - 1];
                }
            }
        }

        Self { dp }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut result = self.dp[row2 as usize][col2 as usize];
        if row1 > 0 {
            result -= self.dp[row1 as usize - 1][col2 as usize];
        }
        if col1 > 0 {
            result -= self.dp[row2 as usize][col1 as usize - 1];
        }

        // 二重に引いた分を足す
        if row1 > 0 && col1 > 0 {
            result += self.dp[row1 as usize - 1][col1 as usize - 1];
        }

        result
    }
}

fn main() {
    let case_1 = (
        vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ],
        [2, 1, 4, 3], // => 8
        [1, 1, 2, 2], // => 11
        [1, 2, 2, 4], // => 12
    );
    // 自分の解答
    let n_mat_0 = NumMatrix::new(case_1.0.clone());
    println!(
        "query_1: {}",
        n_mat_0.sum_region(case_1.1[0], case_1.1[1], case_1.1[2], case_1.1[3])
    );
    println!(
        "query_2: {}",
        n_mat_0.sum_region(case_1.2[0], case_1.2[1], case_1.2[2], case_1.2[3])
    );
    println!(
        "query_3: {}",
        n_mat_0.sum_region(case_1.3[0], case_1.3[1], case_1.3[2], case_1.3[3])
    );

    // Pythonの模範解答
    let n_mat_py = NumMatrixPy::new(case_1.0.clone());
    println!(
        "query_1: {}",
        n_mat_py.sum_region(case_1.1[0], case_1.1[1], case_1.1[2], case_1.1[3])
    );
    println!(
        "query_2: {}",
        n_mat_py.sum_region(case_1.2[0], case_1.2[1], case_1.2[2], case_1.2[3])
    );
    println!(
        "query_3: {}",
        n_mat_py.sum_region(case_1.3[0], case_1.3[1], case_1.3[2], case_1.3[3])
    );

    // C++の模範解答
    let n_mat_cpp = NumMatrixCpp::new(case_1.0.clone());
    println!(
        "query_1: {}",
        n_mat_cpp.sum_region(case_1.1[0], case_1.1[1], case_1.1[2], case_1.1[3])
    );
    println!(
        "query_2: {}",
        n_mat_cpp.sum_region(case_1.2[0], case_1.2[1], case_1.2[2], case_1.2[3])
    );
    println!(
        "query_3: {}",
        n_mat_cpp.sum_region(case_1.3[0], case_1.3[1], case_1.3[2], case_1.3[3])
    );
}
