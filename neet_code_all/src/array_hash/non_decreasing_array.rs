// 解けなかった
struct Solution {}
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut num_dec = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                num_dec += 1;
            }
        }

        num_dec == 1
    }
}

// 模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut changed = false;

        for i in 0..nums.len() - 1 {
            if nums[i] <= nums[i + 1] {
                continue;
            }
            if changed {
                return false;
            }

            if i == 0 || nums[i + 1] >= nums[i - 1] {
                nums[i] = nums[i + 1];
            } else {
                nums[i + 1] = nums[i];
            }
            changed = true;
        }

        true
    }
}

fn main() {
    let case_1 = vec![4, 2, 3];
    // => true
    let case_2 = vec![4, 2, 1];
    // => false

    /*
    println!("case_1: {}", Solution::check_possibility(case_1.clone()));
    println!("case_2: {}", Solution::check_possibility(case_2.clone()));
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_possibility(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_possibility(case_2.clone())
    );
}
