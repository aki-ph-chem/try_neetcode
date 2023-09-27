// 自分の回答
struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_nums = nums.len();
        let mut res = vec![];
        for i in 0..len_nums {
            for j in (i+1)..len_nums {
                if nums[i] + nums[j] == target {
                    res.push(i as i32);
                    res.push(j as i32);
                }
            }
        }
        res
    }
}

use std::collections::HashMap;
// 模範解答
struct SolutionAns {}
impl SolutionAns {
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, n) in nums.into_iter().enumerate(){
            let diff = target - n;
            
            if let Some(&j) = map.get(&diff){
                return vec![i as i32, j as i32];
            }else{
                map.insert(n, i);
            }
        }
        
        unreachable!()
    }
}

fn main() {
    // 自分の解答
    let nums_1 = vec![2,7,11,15];
    let target_1 = 9;
    println!("output: {:?}", Solution::two_sum(nums_1, target_1));

    let nums_2 = vec![3,2,4];
    let target_2 = 6;
    println!("output: {:?}", Solution::two_sum(nums_2, target_2));

    let nums_3 = vec![3,3];
    let target_3 = 6;
    println!("output: {:?}", Solution::two_sum(nums_3, target_3));

    // 模範解答
    let nums_1 = vec![2,7,11,15];
    let target_1 = 9;
    println!("output: {:?}", SolutionAns::two_sum(nums_1, target_1));

    let nums_2 = vec![3,2,4];
    let target_2 = 6;
    println!("output: {:?}", SolutionAns::two_sum(nums_2, target_2));

    let nums_3 = vec![3,3];
    let target_3 = 6;
    println!("output: {:?}", SolutionAns::two_sum(nums_3, target_3));
}
