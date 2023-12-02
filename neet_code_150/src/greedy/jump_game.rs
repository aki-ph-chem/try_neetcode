// 初見では解けなかった
struct Solution {}
impl Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        let mut position = 0;
        for _i in 0..nums.len() {
            if position < nums.len() - 1 {
                position += nums[position] as usize;
            } else {
                break;
            }
            println!("position: {}", position);
        }

        position >= nums.len() - 1
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    fn can_jump(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() - 1;

        for i in (0..goal).rev() {
            // i番目から+ num[i]した位置にジャンプして goal以上ならば
            // goalをi番目に変更
            if i + nums[i] as usize >= goal {
                goal = i;
            }
        }

        // 最終的にgoalが0になったらok
        goal == 0
    }
}

fn main() {
    let case_1 = vec![2, 3, 1, 1, 4];
    let case_2 = vec![3, 2, 1, 0, 4];
    let case_3 = vec![2, 0];
    let case_4 = vec![2,5,0,0];

    println!("{}", "Solution:間違い");
    println!("{}", Solution::can_jump(case_1.clone()));
    println!("{}", Solution::can_jump(case_2.clone()));
    println!("{}", Solution::can_jump(case_3.clone()));
    println!("{}", Solution::can_jump(case_4.clone()));

    println!("{}", "SolutionAns");
    println!("{}", SolutionAns::can_jump(case_1.clone()));
    println!("{}", SolutionAns::can_jump(case_2.clone()));
    println!("{}", SolutionAns::can_jump(case_3.clone()));
    println!("{}", SolutionAns::can_jump(case_4.clone()));
}
