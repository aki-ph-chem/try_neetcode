struct Solution {}
impl Solution {
    // need O(log(n)) time
    // AC
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        let mut mid = (left + right) / 2;
        while left < right {
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }

        nums[mid]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let length = nums.len();

        match length {
            1 => return nums[0],
            _ => (),
        }

        let (mut l, mut r) = (0, length - 1);

        while l < r {
            let m = (l + r) / 2;

            let left = nums[l];
            let mid = nums[m];
            let right = nums[r];

            if left <= mid && mid <= right {
                return left;
            } else if left >= mid && mid >= right {
                return right;
            } else if left > mid || mid < right {
                r = m;
            } else {
                l = m;
            }
        }
        -1
    }
}

fn main() {
    let case_1 = vec![3, 4, 5, 1, 2];
    let case_2 = vec![4, 5, 6, 7, 0, 1, 2];
    let case_3 = vec![11, 13, 15, 17];
    let case_4 = vec![2, 3, 4, 5, 1, 2];
    let case_5 = vec![2, 3];
    let case_6 = vec![3, 2];
    let case_7 = vec![3];
    let case_8 = vec![3, 4, 5, 1, 1, 2];

    println!("Solution::find_min()");
    println!("case_1: {}", Solution::find_min(case_1.clone()));
    println!("case_2: {}", Solution::find_min(case_2.clone()));
    println!("case_3: {}", Solution::find_min(case_3.clone()));
    println!("case_4: {}", Solution::find_min(case_4.clone()));
    println!("case_5: {}", Solution::find_min(case_5.clone()));
    println!("case_6: {}", Solution::find_min(case_6.clone()));
    println!("case_7: {}", Solution::find_min(case_7.clone()));
    println!("case_8: {}", Solution::find_min(case_8.clone()));

    println!("SolutionAns::find_min()");
    println!("case_1: {}", SolutionAns::find_min(case_1.clone()));
    println!("case_2: {}", SolutionAns::find_min(case_2.clone()));
    println!("case_3: {}", SolutionAns::find_min(case_3.clone()));
    println!("case_4: {}", SolutionAns::find_min(case_4.clone()));
    println!("case_5: {}", SolutionAns::find_min(case_5.clone()));
    println!("case_6: {}", SolutionAns::find_min(case_6.clone()));
    println!("case_7: {}", SolutionAns::find_min(case_7.clone()));
    println!("case_8: {}", SolutionAns::find_min(case_8.clone()));
}
