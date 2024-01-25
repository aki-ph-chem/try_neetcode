struct Solution {}
impl Solution {
    // ビット演算では解けなかった
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut bits = 0;
        for v in &nums {
            bits += 1 << *v;
        }

        //println!("{:032b}", bits);

        for i in 0..nums.len() {
            if bits & (1 << i) == 0 {
                return i as i32;
            }
        }

        nums.len() as i32
    }

    // AC(ビット演算ではないが)
    // O(NlogN)
    pub fn missing_number_ln(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();

        for i in 0..n {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }

        return n as i32;
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let length = nums.len() as i32;
        let mut ans = length;
        for i in 0..length {
            ans ^= i ^ nums[i as usize];
        }
        ans
    }
}

fn main() {
    let case_1 = vec![3, 0, 1];
    // => 2
    let case_2 = vec![0, 1];
    // => 2
    let case_3 = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    // => 8
    let case_4 = vec![
        45, 35, 38, 13, 12, 23, 48, 15, 44, 21, 43, 26, 6, 37, 1, 19, 22, 3, 11, 32, 4, 16, 28, 49,
        29, 36, 33, 8, 9, 39, 46, 17, 41, 7, 2, 5, 27, 20, 40, 34, 30, 25, 47, 0, 31, 42, 24, 10,
        14,
    ];
    // => 18

    println!("case_1: {}", Solution::missing_number_ln(case_1.clone()));
    println!("case_2: {}", Solution::missing_number_ln(case_2.clone()));
    println!("case_3: {}", Solution::missing_number_ln(case_3.clone()));
    println!("case_4: {}", Solution::missing_number_ln(case_4.clone()));

    println!("case_1: {}", Solution::missing_number(case_1.clone()));
    println!("case_2: {}", Solution::missing_number(case_2.clone()));
    println!("case_3: {}", Solution::missing_number(case_3.clone()));
    //println!("case_4: {}", Solution::missing_number(case_4.clone()));
    //overflow
}
