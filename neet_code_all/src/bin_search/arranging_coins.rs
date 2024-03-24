struct Solution {}
impl Solution {
    // AC
    // 直線探索
    pub fn arrange_coins_linear(n: i32) -> i32 {
        let n = n as i64;
        let sum_n = |m: i64| m * (m + 1) / 2;

        let mut m = 0;
        loop {
            if n == sum_n(m) {
                return m as i32;
            }
            if n < sum_n(m) {
                m -= 1;
                break;
            }

            m += 1;
        }

        m as i32
    }

    // AC
    // 二分探索
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let sum_n = |m: i64| m * (m + 1) / 2;

        let (mut left, mut mid, mut right) = (0, 0, n);
        while left <= right {
            mid = left + (right - left) / 2;

            if n == sum_n(mid) {
                return mid as i32;
            }

            if n < sum_n(mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if n < sum_n(mid) {
            return mid as i32 - 1;
        }

        mid as i32
    }
}

// C++の模範解答より
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let (mut left, mut right, mut result) = (1_i64, n as i64, 1_i64);
        let (mut sum, mut m) = (0_i64, 0_i64);

        while left <= right {
            m = left + (right - left) / 2;

            sum = m * (m + 1) / 2;

            if sum == n {
                return m as i32;
            }

            if n < sum {
                right = m - 1;
            } else {
                result = m;
                left = m + 1;
            }
        }

        result as i32
    }
}

fn main() {
    let case_1 = 5;
    // => 2
    let case_2 = 8;
    // => 3
    let case_3 = 1804289383;

    println!("case_1: {}", Solution::arrange_coins_linear(case_1));
    println!("case_2: {}", Solution::arrange_coins_linear(case_2));
    println!("case_3: {}", Solution::arrange_coins_linear(case_3));

    println!("case_1: {}", Solution::arrange_coins(case_1));
    println!("case_2: {}", Solution::arrange_coins(case_2));
    println!("case_3: {}", Solution::arrange_coins(case_3));

    println!("case_1: {}", SolutionAnsCpp::arrange_coins(case_1));
    println!("case_2: {}", SolutionAnsCpp::arrange_coins(case_2));
    println!("case_3: {}", SolutionAnsCpp::arrange_coins(case_3));
}
