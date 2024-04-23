// 和が最大となるような連続部分配列の和を返す
pub fn max_subarray_sum(array: &Vec<i32>) -> i32 {
    let (mut max, mut sum_positive) = (i32::MIN, 0);
    for v in array {
        sum_positive += v;
        max = max.max(sum_positive);

        if sum_positive < 0 {
            sum_positive = 0;
        }
    }

    max
}

// 和が最大となるような連続配列の範囲をタプル((start,end), max_value)で返す
pub fn max_sub_array_sum_range(array: &Vec<i32>) -> ((i32, i32), i32) {
    let (mut start, mut end) = (0, 0);
    let (mut sum_idx, mut sum_positive, mut max) = (0, 0, i32::MIN);

    for (idx, v) in array.iter().enumerate() {
        sum_positive += v;

        if max < sum_positive {
            (start, end, max) = (sum_idx, idx, sum_positive);
        }

        if sum_positive < 0 {
            sum_positive = 0;
            sum_idx = idx + 1;
        }
    }

    ((start as i32, end as i32), max)
}

// DP
pub fn max_sub_array_sum_dp(array: &Vec<i32>) -> i32 {
    let n = array.len();
    let mut dp = vec![0; n];

    dp[0] = array[0];
    let mut result = dp[0];
    for i in 1..n {
        dp[i] = array[i].max(array[i] + dp[i - 1]);
        result = result.max(dp[i]);
    }

    result
}

fn main() {
    let case_1 = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    // => max: 7, range: (2,6)

    println!("case_1: {}", max_subarray_sum(&case_1));
    println!("case_1: {:?}", max_sub_array_sum_range(&case_1));
    println!("case_1: {}", max_sub_array_sum_dp(&case_1));
}
