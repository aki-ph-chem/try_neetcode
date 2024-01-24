// AC
struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize + 1];

        for i in 0..=n {
            result[i as usize] = Self::count(i);
        }

        result
    }

    fn count(mut n: i32) -> i32 {
        let mut res = 0;

        while n != 0 {
            let bit = n & 1;
            if bit == 1 {
                res += 1;
            }

            n = n >> 1;
        }

        res
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for i in 0..=n {
            ans.push(Self::set_bits(i));
        }

        ans
    }

    fn set_bits(mut n: i32) -> i32 {
        let mut count = 0;

        while n > 0 {
            n = n & (n - 1);
            count += 1;
        }

        count
    }
}

fn main() {
    let case_1 = 2;
    // => [0,1,1]
    let case_2 = 5;
    // => [0,1,1,2,1,2]

    println!("case_1: {:?}", Solution::count_bits(case_1));
    println!("case_2: {:?}", Solution::count_bits(case_2));

    println!("case_1: {:?}", SolutionAns::count_bits(case_1));
    println!("case_2: {:?}", SolutionAns::count_bits(case_2));
}
