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

            if nums[mid] < nums[right] {}
        }

        -1
    }
}

fn main() {
    let case_1 = (vec![4, 5, 6, 7, 0, 1, 2], 0);
    let case_2 = (vec![4, 5, 6, 7, 0, 1, 2], 3);
    let case_3 = (vec![1], 0);
}
