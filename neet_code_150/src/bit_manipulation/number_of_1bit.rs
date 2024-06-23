//AC
struct Solution {}
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut sum = 0;
        for i in 0..32 {
            if n & (1 << i) != 0 {
                sum += 1;
            }
        }

        sum as i32
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut count = 0;

        while n > 0 {
            n = n & (n - 1);
            count += 1;
        }

        count
    }
}

// 後で解いた別解
// AC
struct SolutionLatter;
impl SolutionLatter {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 0 {
            if n & 1 != 0 {
                result += 1;
            }

            n = n >> 1;
        }

        result
    }
}

// C++の模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut result = 0;

        while n != 0 {
            let bit = n & 1;
            if bit == 1 {
                result += 1;
            }

            n = n >> 1;
        }

        result
    }
}

fn main() {
    let case_1 = 0b00000000000000000000000000001011_u32;
    // => 3
    let case_2 = 0b00000000000000000000000010000000_u32;
    // => 1
    let case_3 = 0b11111111111111111111111111111101_u32;
    // => 31

    println!("case_1: {}", Solution::hamming_weight(case_1));
    println!("case_2: {}", Solution::hamming_weight(case_2));
    println!("case_3: {}", Solution::hamming_weight(case_3));

    println!("case_1: {}", SolutionAns::hamming_weight(case_1));
    println!("case_2: {}", SolutionAns::hamming_weight(case_2));
    println!("case_3: {}", SolutionAns::hamming_weight(case_3));

    println!("case_1: {}", SolutionAnsCpp::hamming_weight(case_1));
    println!("case_2: {}", SolutionAnsCpp::hamming_weight(case_2));
    println!("case_3: {}", SolutionAnsCpp::hamming_weight(case_3));

    println!("case_1: {}", SolutionLatter::hamming_weight(case_1));
    println!("case_2: {}", SolutionLatter::hamming_weight(case_2));
    println!("case_3: {}", SolutionLatter::hamming_weight(case_3));
}
