struct Solution {}
impl Solution {
// 現状: trueを返す場合ではloopから抜けれるが
// falseを返す場合無限ループになる
    fn can_jump(nums: Vec<i32>) -> bool {
        let mut position = 0;
        while position < nums.len() - 1 {
            println!("position: {}", position);
            position += nums[position] as usize;
        }

        position == nums.len() - 1
    }
}

fn main() {
    let case_1 = vec![2,3,1,1,4];
    let case_2 = vec![3,2,1,0,4];

    println!("{}",Solution::can_jump(case_1.clone()));
    println!("{}",Solution::can_jump(case_2.clone()));
}
