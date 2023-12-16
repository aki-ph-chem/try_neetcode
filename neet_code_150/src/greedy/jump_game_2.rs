// C++の模範解答より
struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        let mut i = 0;
        while i < n - 1 {
            if i + nums[i] as usize >= n - 1 {
                result += 1;
                break;
            }

            let (mut max_idx, mut max_v) = (i + 1, 0);
            for j in (i + 1)..(i + 1 + nums[i] as usize) {
                if j as i32 + nums[j] > max_v {
                    max_idx = j;
                    max_v = j as i32 + nums[j];
                }
            }
            i = max_idx;
            result += 1;
        }

        result
    }
}

fn main() {
    let case_1 = vec![2, 3, 1, 1, 4];
    let case_2 = vec![3, 2, 1, 0, 4];
    let case_3 = vec![2, 0];
    let case_4 = vec![2, 5, 0, 0];

    println!("case_1: {}", Solution::jump(case_1));
    println!("case_1: {}", Solution::jump(case_2));
    println!("case_1: {}", Solution::jump(case_3));
    println!("case_1: {}", Solution::jump(case_4));
}
