// 和が最大となるような連続部分配列の和を返す
pub fn max_subarray_sum(array: &Vec<i32>) -> i32 {
    let (mut max, mut sum_current) = (i32::MIN, 0);
    for v in array {
        sum_current += v;
        max = max.max(sum_current);

        if sum_current < 0 {
            sum_current = 0;
        }
    }

    max
}

// 上を少し工夫してスッキリさせた
pub fn max_subarray_sum_2(array: &Vec<i32>) -> i32 {
    let (mut max, mut sum_current) = (i32::MIN, 0);
    for v in array {
        sum_current = std::cmp::max(*v, sum_current + *v);
        max = max.max(sum_current);
    }

    max
}

// 和が最大となるような連続配列の範囲をタプル((start,end), max_value)で返す
pub fn max_sub_array_sum_range(array: &Vec<i32>) -> ((i32, i32), i32) {
    let (mut start, mut end) = (0, 0);
    let (mut sum_idx, mut sum_current, mut max) = (0, 0, i32::MIN);

    for (idx, v) in array.iter().enumerate() {
        sum_current += v;

        if max < sum_current {
            (start, end, max) = (sum_idx, idx, sum_current);
        }

        if sum_current < 0 {
            sum_current = 0;
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
    let case_2 = vec![-2, -3, -4, -1, -2, -1, -5, -3];
    // => max: -1,range(3,3)or range(6,6)

    println!("case_1: {}", max_subarray_sum(&case_1));
    println!("case_1: {}", max_subarray_sum_2(&case_1));
    println!("case_1: {:?}", max_sub_array_sum_range(&case_1));
    println!("case_1: {}", max_sub_array_sum_dp(&case_1));

    println!("case_2: {}", max_subarray_sum(&case_2));
    println!("case_2: {}", max_subarray_sum_2(&case_2));
    println!("case_2: {:?}", max_sub_array_sum_range(&case_2));
    println!("case_2: {}", max_sub_array_sum_dp(&case_2));
}
