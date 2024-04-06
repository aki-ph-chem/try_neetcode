// 解けなかった
struct Solution {}
impl Solution {
    // search in rotated sorted arrayの回答を移植したけどダメ(WA)だった..
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len() as i32;
        let (mut left, mut right) = (0, n as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return true;
            }

            if nums[left as usize] <= nums[mid as usize] {
                if target < nums[left as usize] || target > nums[mid as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if target < nums[mid as usize] || target > nums[right as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        false
    }
}

// Kotlinの模範解答より
struct SolutionAnsKotlin {}
impl SolutionAnsKotlin {
    // AC
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return true;
            }

            if nums[left as usize] < nums[mid as usize] {
                if nums[left] <= target && nums[mid as usize] > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if nums[left as usize] > nums[mid as usize] {
                if nums[right as usize] >= target && nums[mid as usize] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                left += 1;
            }
        }

        false
    }
}

fn main() {
    let case_1 = (vec![2, 5, 6, 0, 0, 1, 2], 0);
    // => true
    let case_2 = (vec![2, 5, 6, 0, 0, 1, 2], 3);
    // => false
    let case_3 = (vec![1, 0, 1, 1, 1], 0);
    // => true

    /*
    println!("case_1: {}", Solution::search(case_1.0.clone(), case_1.1));
    println!("case_2: {}", Solution::search(case_2.0.clone(), case_2.1));
    println!("case_3: {}", Solution::search(case_3.0.clone(), case_3.1));
    */

    println!(
        "case_1: {}",
        SolutionAnsKotlin::search(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::search(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsKotlin::search(case_3.0.clone(), case_3.1)
    );
}
