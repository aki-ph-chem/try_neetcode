struct Solution {}
impl Solution {
    // O(n)だと TLE
    pub fn pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        if n >= 0 {
            for _i in 0..n {
                res *= x;
            }
        } else {
            for _i in 0..(-n) {
                res /= x;
            }
        }

        res
    }

    // AC
    // 再帰で解く
    pub fn pow_b(x: f64, n: i32) -> f64 {
        if n >= 0 {
            Self::pow_raw(x, n)
        } else {
            1.0 / Self::pow_raw(x, n)
        }
    }

    fn pow_raw(x: f64, n: i32) -> f64 {
        match n.abs() {
            0 => 1.0,
            1 => x,
            _ => {
                if n % 2 == 0 {
                    Self::pow_raw(x, n / 2) * Self::pow_raw(x, n / 2)
                } else {
                    x * Self::pow_raw(x, n / 2) * Self::pow_raw(x, n / 2)
                }
            }
        }
    }
}

// C++の模範解答をRustに直したもの
// なぜかACしない
// O(log(n))
struct SolutionAns {}
impl SolutionAns {
    pub fn pow(x: f64, n: i32) -> f64 {
        let exponent = n.abs() as i64;
        let mut current = x;
        let mut result = 1.0;

        let mut i = exponent;
        while i > 0 {
            if i % 2 == 1 {
                result *= current;
            }
            current *= current;

            i /= 2;
        }

        if n >= 0 {
            result
        } else {
            1.0 / result
        }
    }
}

fn main() {
    let case_1 = (2.00000, 10); // 1024.000
    let case_2 = (2.10000, 3); // 9.26100
    let case_3 = (2.00000, -2); // 0.25
    let case_4 = (2.00000, 3); // 8

    println!("case_1: {}", Solution::pow(case_1.0, case_1.1));
    println!("case_2: {}", Solution::pow(case_2.0, case_2.1));
    println!("case_3: {}", Solution::pow(case_3.0, case_3.1));

    println!("case_1: {}", Solution::pow_b(case_1.0, case_1.1));
    println!("case_2: {}", Solution::pow_b(case_2.0, case_2.1));
    println!("case_3: {}", Solution::pow_b(case_3.0, case_3.1));
    println!("case_4: {}", Solution::pow_b(case_4.0, case_4.1));

    // 模範解答
    println!("case_1: {}", SolutionAns::pow(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::pow(case_2.0, case_2.1));
    println!("case_3: {}", SolutionAns::pow(case_3.0, case_3.1));
    println!("case_4: {}", SolutionAns::pow(case_4.0, case_4.1));
}
