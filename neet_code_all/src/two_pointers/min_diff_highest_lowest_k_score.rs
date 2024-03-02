struct Solution {}
impl Solution {
    // AC
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len() as i32;

        let mut result = i32::MAX;
        for i in 0..n {
            if i + k - 1 < n {
                //println!("{}", nums[(i + k - 1) as usize] - nums[i as usize]);
                result = result.min(nums[(i + k - 1) as usize] - nums[i as usize]);
            } else {
                break;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // 解1
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut left, mut right) = (0, (k - 1) as usize);
        let mut result = i32::MAX;

        while right < nums.len() {
            result = result.min(nums[right] - nums[left]);
            left += 1;
            right += 1;
        }

        result
    }

    // 解2
    fn minimum_difference_idiomatic(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums.windows(k as usize)
            .map(|pair| pair[(k - 1) as usize] - pair[0])
            .min()
            .unwrap()
    }
}

//AC
//C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut l, mut r) = (0, k - 1);
        let mut result = i32::MAX;

        while r < nums.len() as i32 {
            result = result.min(nums[r as usize] - nums[l]);
            l += 1;
            r += 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![90], 1);
    // => 0
    let case_2 = (vec![9, 4, 1, 7], 2);
    // => 2

    println!(
        "case_1: {}",
        Solution::minimum_difference(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::minimum_difference(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::minimum_difference(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::minimum_difference(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::minimum_difference_idiomatic(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::minimum_difference_idiomatic(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::minimum_difference(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::minimum_difference(case_2.0.clone(), case_2.1)
    );
}
