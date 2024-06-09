// 解けなかった
struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        1
    }

    pub fn trap_sq(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut result = 0;

        for i in 0..n {}

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);
        let mut result = 0;

        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                result += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                result += right_max - height[right];
            }
        }
        result
    }
}

// space: O(n), time: O(n) latter
struct SolutionLatter;
impl SolutionLatter {
    // AC
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if height.is_empty() {
            return 0;
        }

        let mut max_left = vec![i32::MIN; n + 1];
        let mut max_right = vec![i32::MIN; n + 1];
        for i in 0..n {
            max_left[i + 1] = max_left[i].max(height[i]);
            max_right[n - 1 - i] = max_right[n - i].max(height[n - 1 - i]);
        }

        /*
        println!("max_left: {:?}", max_left);
        println!("max_right: {:?}", max_right);
        */
        let mut result = 0;
        for i in 0..n {
            result += (max_left[i + 1].min(max_right[i]) - height[i]).max(0);
        }

        result
    }
}

fn main() {
    let case_1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    // => 6
    let case_2 = vec![4, 2, 0, 3, 2, 5];
    // => 9
    let case_3 = vec![0, 1, 0, 0, 1, 0, 1];
    // => 3

    println!("case_1: {}", SolutionAns::trap(case_1.clone()));
    println!("case_2: {}", SolutionAns::trap(case_2.clone()));
    println!("case_3: {}", SolutionAns::trap(case_3.clone()));

    println!("case_1: {}", SolutionLatter::trap(case_1.clone()));
    println!("case_2: {}", SolutionLatter::trap(case_2.clone()));
    println!("case_3: {}", SolutionLatter::trap(case_3.clone()));

    println!("case_1: {}", SolutionLatter::trap_2(case_1.clone()));
    /*
    println!("case_2: {}", SolutionLatter::trap_2(case_2.clone()));
    println!("case_3: {}", SolutionLatter::trap_2(case_3.clone()));
    */
}
