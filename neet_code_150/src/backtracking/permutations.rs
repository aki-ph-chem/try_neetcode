// 解けなかった
struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];

        for i in 0..n {
            Self::dfs(&nums, i as i32, &mut result);
        }

        result
    }

    fn dfs(nums: &Vec<i32>, start: i32, result: &mut Vec<Vec<i32>>) {}
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        Self::backtrack(0, &mut result, &mut nums);

        result
    }

    fn backtrack(first: usize, result: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>) {
        if first == nums.len() {
            result.push(nums.clone());
            return;
        }

        for i in first..nums.len() {
            nums.swap(first, i);
            Self::backtrack(first + 1, result, nums);
            nums.swap(first, i);
        }
    }
}

fn main() {
    let case_1 = vec![1, 2, 3];
    let case_2 = vec![0, 1];

    /*
    println!("case_1 : {:?}", Solution::permute(case_1.clone()));
    println!("case_2 : {:?}", Solution::permute(case_2.clone()));
    */

    println!("case_1 : {:?}", SolutionAns::permute(case_1.clone()));
    println!("case_2 : {:?}", SolutionAns::permute(case_2.clone()));
}
