struct Solution {}
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        // AC
        let sign_func = |x: i32| match x {
            0 => 0,
            _ => x / x.max(-x),
        };

        nums.iter().fold(1, |prod, x| prod * sign_func(*x))
    }

    // AC
    pub fn array_sign_2(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |prod, x| {
            prod * match x {
                0 => 0,
                _ => x / x.max(&-x),
            }
        })
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut num_neg = 0;
        for v in nums {
            if v == 0 {
                return 0;
            } else if v < 0 {
                num_neg += 1;
            }
        }

        if num_neg & 1 != 0 {
            return -1;
        }

        1
    }
}

fn main() {
    let case_1 = vec![-1, -2, -3, -4, 3, 2, 1];
    // => 1
    let case_2 = vec![1, 5, 0, 2, -3];
    // => 0
    let case_3 = vec![-1, 1, -1, 1, -1];
    // => -1

    println!("case_1: {}", Solution::array_sign(case_1.clone()));
    println!("case_2: {}", Solution::array_sign(case_2.clone()));
    println!("case_3: {}", Solution::array_sign(case_3.clone()));

    println!("case_1: {}", Solution::array_sign_2(case_1.clone()));
    println!("case_2: {}", Solution::array_sign_2(case_2.clone()));
    println!("case_3: {}", Solution::array_sign_2(case_3.clone()));

    println!("case_1: {}", SolutionAnsCpp::array_sign(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::array_sign(case_2.clone()));
    println!("case_3: {}", SolutionAnsCpp::array_sign(case_3.clone()));
}
