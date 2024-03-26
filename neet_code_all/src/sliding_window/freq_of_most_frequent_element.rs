// 解けなかった(全く方針が立たなかった)
struct Solution {}
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        //nums.sort();

        let mut result = 0;
        for i in 0..n {}

        result
    }
}

// 模範解答 => WAとなって正しくない
struct SolutionAns {}
impl SolutionAns {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let (mut left, mut right, mut res, mut total) = (0, 0, 0, 0);

        nums.sort_unstable();

        while right < nums.len() {
            total += nums[right];

            while (nums[right] * (right - left + 1) as i32) - total > k {
                total -= nums[left];
                left += 1;
            }

            res = res.max(right - left + 1);
            right += 1;
        }

        res as i32
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut nums = nums;
        nums.sort();

        let (mut left, mut right) = (0, 0);
        let mut total = 0_i64;
        let mut result = 0;

        while right < nums.len() {
            total += nums[right] as i64;

            while (right - left + 1) as i64 * nums[right] as i64 > total + k {
                total -= nums[left] as i64;
                left += 1;
            }

            result = result.max(right - left + 1);
            right += 1;
        }

        result as i32
    }
}

fn main() {
    let case_1 = (vec![1, 2, 4], 5);
    // => 3
    let case_2 = (vec![1, 4, 8, 13], 5);
    // => 2
    let case_3 = (vec![3, 9, 6], 2);
    // => 1

    /*
    println!(
        "case_1: {}",
        SolutionAns::max_frequency(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::max_frequency(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::max_frequency(case_3.0.clone(), case_3.1)
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::max_frequency(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::max_frequency(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::max_frequency(case_3.0.clone(), case_3.1)
    );
}
