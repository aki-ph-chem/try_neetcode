struct Solution;
impl Solution {
    // AC
    pub fn is_ugly(n: i32) -> bool {
        if n < 0 {
            return false;
        }
        let mut n = n;

        while n != 0 && n % 2 == 0 {
            n /= 2;
        }
        while n != 0 && n % 3 == 0 {
            n /= 3;
        }
        while n != 0 && n % 5 == 0 {
            n /= 5;
        }

        n == 1
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn is_ugly(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let mut n = n;

        for p in [2, 3, 5] {
            while n % p == 0 {
                n /= p;
            }
        }

        n == 1
    }
}

fn main() {
    let case_1 = 6;
    let case_2 = 1;
    let case_3 = 14;

    println!("case_1: {}", Solution::is_ugly(case_1));
    println!("case_2: {}", Solution::is_ugly(case_2));
    println!("case_3: {}", Solution::is_ugly(case_3));

    println!("case_1: {}", SolutionAns::is_ugly(case_1));
    println!("case_2: {}", SolutionAns::is_ugly(case_2));
    println!("case_3: {}", SolutionAns::is_ugly(case_3));
}
