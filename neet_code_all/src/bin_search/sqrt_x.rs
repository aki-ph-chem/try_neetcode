// 解けなかった
// 浮動小数点で処理して後で小数点以下切り捨て-> 条件がシビア
// y = x^2 - n のゼロ点として sqrt(n)を求める方針がダメだったみたい
//
struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let (mut left, mut right) = (0, x);
        let sq_func = |y| y * y - x;

        let mut mid = 0;
        while left <= right {
            mid = left + (right - left) / 2;

            if sq_func(mid) * sq_func(right) > 0 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        mid as i32
    }

    pub fn my_sqrt_2(x: i32) -> i32 {
        if x == 1 {
            return 1;
        }

        let x = x as f64;
        let (mut left, mut right) = (0.0, x);
        let sq_func = |y| y * y - x;

        let mut mid = 0.0;
        let epsilon = 1e-3;
        while (right - left).abs() > epsilon {
            mid = left + (right - left) / 2.0;

            if sq_func(mid) * sq_func(right) > 0.0 {
                right = mid;
            } else {
                left = mid;
            }
        }

        //println!("mid: {:?}", mid);
        mid.floor() as i32
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let x = x as i64;
        let (mut left, mut mid, mut right) = (0_i64, 0_i64, x as i64 / 2);
        while left <= right {
            mid = (left + right) / 2;

            if mid.pow(2) == x {
                return mid as i32;
            }

            if mid.pow(2) < x {
                if (mid + 1).pow(2) > x {
                    return mid as i32;
                }
                left = mid + 1;
            } else {
                if (mid - 1).pow(2) < x {
                    return mid as i32 - 1;
                }

                right = mid - 1;
            }
        }

        mid as i32
    }
}

fn main() {
    let case_1 = 4;
    // => 2
    let case_2 = 8;
    // => 3
    let case_3 = 2147395599;
    // => 46339
    let case_4 = 1;
    // => 1
    let case_5 = 36;
    // => 6
    let case_6 = 9;
    // => 3

    println!("case_1: {}", Solution::my_sqrt(case_1));
    println!("case_2: {}", Solution::my_sqrt(case_2));
    //println!("case_3: {}", Solution::my_sqrt(case_3));
    println!("case_4: {}", Solution::my_sqrt(case_4));
    println!("case_5: {}", Solution::my_sqrt(case_5));

    println!("case_1: {}", Solution::my_sqrt_2(case_1));
    println!("case_2: {}", Solution::my_sqrt_2(case_2));
    println!("case_3: {}", Solution::my_sqrt_2(case_3));
    println!("case_4: {}", Solution::my_sqrt_2(case_4));
    println!("case_5: {}", Solution::my_sqrt_2(case_5));
    println!("case_6: {}", Solution::my_sqrt_2(case_6));

    println!("case_1: {}", SolutionAnsCpp::my_sqrt(case_1));
    println!("case_2: {}", SolutionAnsCpp::my_sqrt(case_2));
    println!("case_3: {}", SolutionAnsCpp::my_sqrt(case_3));
    println!("case_4: {}", SolutionAnsCpp::my_sqrt(case_4));
    println!("case_5: {}", SolutionAnsCpp::my_sqrt(case_5));
    println!("case_6: {}", SolutionAnsCpp::my_sqrt(case_6));
}
