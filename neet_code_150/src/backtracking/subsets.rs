use std::iter::FromIterator;

// AC
struct Solution {}
impl Solution {
    // O(N2^N)
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];

        for i in 0..(1 << n) {
            let mut sub_set = vec![];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    sub_set.push(nums[j]);
                }
            }
            result.push(sub_set.clone());
        }

        result
    }
}

// AC
// C++の模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut result = vec![];

        Self::dfs(&nums, 0, &mut current, &mut result);

        result
    }

    fn dfs(nums: &Vec<i32>, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());

        for i in start..(nums.len() as i32) {
            current.push(nums[i as usize]);
            Self::dfs(nums, i + 1, current, result);
            current.pop();
        }
    }
}

// 模範解答
// ビット演算とイテレータ
struct SolutionAns {}
impl SolutionAns {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();

        Vec::from_iter((0..1 << n).map(|bitmask| {
            Vec::from_iter((0..n).filter_map(|i| match (bitmask >> i) & 1 != 0 {
                true => Some(nums[i]),
                false => None,
            }))
        }))
    }
}

fn main() {
    let case_1 = vec![1, 2, 3];
    let case_2 = vec![0];

    println!("case_1: {:?}", Solution::subsets(case_1.clone()));
    println!("case_2: {:?}", Solution::subsets(case_2.clone()));

    println!("case_1: {:?}", SolutionAnsCpp::subsets(case_1.clone()));
    println!("case_2: {:?}", SolutionAnsCpp::subsets(case_2.clone()));

    println!("case_1: {:?}", SolutionAns::subsets(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::subsets(case_2.clone()));
}
