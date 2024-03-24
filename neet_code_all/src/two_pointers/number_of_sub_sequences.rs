// 解けなかった
struct Solution {}
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut result = 0_i64;
        for i in 0..nums.len() {}

        let mod_by = 10_i64.pow(9) + 7;
        (result % mod_by) as i32
    }

    // シフト演算がオーバーフローするからダメ
    // N2^N
    pub fn num_subseq_bit(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        let mut result = 0_i64;

        for bit in 0..(1 << n) {
            let (mut min_v, mut max_v) = (std::i32::MAX, 0);
            for i in 0..n {
                if bit & (1 << i) != 0 {
                    min_v = min_v.min(nums[i as usize]);
                    max_v = max_v.max(nums[i as usize]);
                }
            }
            if min_v + max_v <= target {
                result += 1;
            }
        }

        let mod_by = 10_i64.pow(9) + 7;
        (result % mod_by) as i32
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        let (mut left, mut right) = (0, n as i32 - 1);
        let mut result = 0_i64;
        let mod_by = 10_i64.pow(9) + 7;

        while left <= right {
            if nums[left as usize] + nums[right as usize] > target {
                right -= 1;
            } else {
                result = (result + Self::fast_power(2, (right - left) as i64, mod_by)) % mod_by;
                left += 1;
            }
        }

        result as i32
    }

    fn fast_power(a: i64, mut b: i64, mod_by: i64) -> i64 {
        let mut result = 1_i64;
        let mut base = a;

        while b != 0 {
            if b % 2 == 1 {
                result = (result * base) % mod_by;
            }
            base = (base * base) % mod_by;
            b /= 2;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![3, 5, 6, 7], 9);
    // 4: ([3], [3,5], [3,5,6], [3,6])
    let case_2 = (vec![3, 3, 6, 8], 10);
    // 6: ([3], [3], [3,3], [3,6], [3,6], [3,3,6])
    let case_3 = (vec![2, 3, 3, 4, 6, 7], 12);
    // 61
    let case_4 = (vec![5, 2, 4, 1, 7, 6, 8], 16);
    // => 127
    let case_5 = (vec![1], 1);

    /*
    println!(
        "case_1: {}",
        Solution::num_subseq_bit(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::num_subseq_bit(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::num_subseq_bit(case_3.0.clone(), case_3.1)
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::num_subseq(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::num_subseq(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::num_subseq(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::num_subseq(case_4.0.clone(), case_4.1)
    );
    println!(
        "case_5: {}",
        SolutionAnsCpp::num_subseq(case_5.0.clone(), case_5.1)
    );
}
