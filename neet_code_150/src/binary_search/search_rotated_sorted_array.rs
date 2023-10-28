// 解けなかった😭
struct Solution {}
impl Solution {
    // need O(long(n)) time
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut mid = (left + right) / 2;
        while left < right {
            if nums[mid] == target {
                return mid as i32;
            }

            if target > nums[mid] && nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }

        -1
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l <= r {
            let m = (l + r) / 2;

            if nums[m] == target {
                return m as i32;
            }

            if nums[l] <= nums[m] {
                if target < nums[l] || target > nums[m] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } else {
                if target < nums[m] || target > nums[r] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }
        }

        -1
    }
}

fn main() {
    let case_1 = (vec![4, 5, 6, 7, 0, 1, 2], 0); // 4
    let case_2 = (vec![4, 5, 6, 7, 0, 1, 2], 3); // -1
    let case_3 = (vec![1], 0); // -1
    let case_4 = (vec![3, 4, 5, 1, 2], 2); // 3
    // 普通の二分探索
    let case_5 = (vec![3, 4, 5], 4); // 1
    let case_6 = (vec![2, 3, 4, 5, 7], 4); // 2

    println!("case_1: {}", SolutionAns::search(case_1.0.clone(), case_1.1));
    println!("case_2: {}", SolutionAns::search(case_2.0.clone(), case_2.1));
    println!("case_3: {}", SolutionAns::search(case_3.0.clone(), case_3.1));
    println!("case_4: {}", SolutionAns::search(case_4.0.clone(), case_4.1));
    println!("case_5: {}", SolutionAns::search(case_5.0.clone(), case_5.1));
    println!("case_6: {}", SolutionAns::search(case_6.0.clone(), case_6.1));
}
