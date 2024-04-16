// 解けなかった
struct Solution {}
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut x = x;
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut postfix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        for i in (0..n).rev() {
            postfix_sum[i] = postfix_sum[i + 1] + nums[i];
        }
        println!("prefix_sum: {:?}", prefix_sum);
        println!("postfix_sum: {:?}", postfix_sum);

        -1
    }
}

// Kotlinの模範解答より
struct SolutionAnsKotlin {}
impl SolutionAnsKotlin {
    // AC
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let (mut left, mut max, mut current) = (0, -1_i32, 0);
        let target = nums.iter().sum::<i32>() - x;

        for right in 0..nums.len() {
            current += nums[right];

            while left <= right && current > target {
                current -= nums[left];
                left += 1;
            }

            if current == target {
                max = max.max(right as i32 - left as i32 + 1);
            }
        }

        if max == -1 {
            return -1;
        }

        nums.len() as i32 - max
    }
}

fn main() {
    let case_1 = (vec![1, 1, 4, 2, 3], 5);
    // => 2
    let case_2 = (vec![5, 6, 7, 8, 9], 4);
    // => -1
    let case_3 = (vec![3, 2, 20, 1, 1, 3], 10);
    // => 5

    println!(
        "case_1: {}",
        SolutionAnsKotlin::min_operations(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::min_operations(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsKotlin::min_operations(case_3.0.clone(), case_3.1)
    );
}
