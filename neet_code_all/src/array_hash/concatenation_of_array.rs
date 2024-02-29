struct Solution {}
impl Solution {
    // AC
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        [nums.clone(), nums].concat()
    }

    // AC(こっちのほうが速い)
    pub fn get_concatenation_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        for v in nums {
            result.push(v);
        }

        result
    }
}

// 模範解答:
struct SolutionAns {}
impl SolutionAns {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; 2 * nums.len()];
        for i in 0..nums.len() {
            result[i] = nums[i];
            result[i + nums.len()] = nums[i];
        }

        result
    }
}

fn main() {
    let case_1 = vec![1, 2, 1];
    let case_2 = vec![1, 3, 2, 1];

    println!("case_1: {:?}", Solution::get_concatenation(case_1.clone()));
    println!("case_2: {:?}", Solution::get_concatenation(case_2.clone()));

    println!(
        "case_1: {:?}",
        SolutionAns::get_concatenation(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::get_concatenation(case_2.clone())
    );
}
