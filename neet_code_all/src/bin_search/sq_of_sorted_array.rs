struct Solution {}
impl Solution {
    // 思いつかなかった
    pub fn soreted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 最初の値が0以上ならただ二乗するだけ
        if nums[0] >= 0 {
            return nums.iter().map(|x| x.pow(2)).collect();
        }

        let mut result = vec![];
        let (mut left, mut mid, mut right) = (0, 0, nums.len() as i32 - 1);

        while left <= right {
            mid = left + (right - left) / 2;
        }

        result
    }

    // AC
    // ナイーブな実装(つまらない)
    pub fn sorted_squares_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.iter().map(|x| x.pow(2)).collect();
        result.sort();

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let (mut idx, mut left, mut right) = (nums.len() as i32 - 1, 0, nums.len() as i32 - 1);

        while left <= right {
            if nums[left as usize].abs() > nums[right as usize].abs() {
                result[idx as usize] = nums[left as usize].pow(2);
                left += 1;
            } else {
                result[idx as usize] = nums[right as usize].pow(2);
                right -= 1;
            }

            idx -= 1;
        }

        result
    }
}

fn main() {
    let case_1 = vec![-4, -1, 0, 3, 10];
    // => [0,1,9,16,100]
    let case_2 = vec![-7, -3, 2, 3, 11];
    // =>  [4,9,9,49,121]
    let case_3 = vec![-5, -3, 2, 10];
    // => [0, 1, 9, 100]

    println!("case_1: {:?}", Solution::sorted_squares_2(case_1.clone()));
    println!("case_2: {:?}", Solution::sorted_squares_2(case_2.clone()));

    println!("case_1: {:?}", SolutionAns::sorted_squares(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::sorted_squares(case_2.clone()));
}
