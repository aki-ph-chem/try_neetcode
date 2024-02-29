struct Solution {}
impl Solution {
    // AC
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut result = vec![0; n];
        result[n - 1] = -1;

        // 後ろから累積的な最大値を計算する
        let mut max_tmp = i32::MIN;
        for i in (1..n).rev() {
            max_tmp = max_tmp.max(arr[i]);
            result[i - 1] = max_tmp;
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut result = vec![-1; n];

        for i in (1..=(n - 1)).rev() {
            result[i - 1] = std::cmp::max(result[i], arr[i]);
        }

        result
    }
}

fn main() {
    let case_1 = vec![17, 18, 5, 4, 6, 1];
    // => [18,6,6,6,1,-1]
    let case_2 = vec![400];
    // => [-1]

    println!("case_1: {:?}", Solution::replace_elements(case_1.clone()));
    println!("case_2: {:?}", Solution::replace_elements(case_2.clone()));

    println!(
        "case_1: {:?}",
        SolutionAns::replace_elements(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::replace_elements(case_2.clone())
    );
}
