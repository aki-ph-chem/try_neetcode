use std::collections::HashSet;

// AC
// ビット演算で部分集合を列挙して、ダブり判定をHashSetで行う
struct Solution {}
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];
        let mut set: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..(1 << n) {
            let mut sub_set = vec![];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    sub_set.push(nums[j]);
                }
            }
            sub_set.sort();
            if !set.contains(&sub_set) {
                set.insert(sub_set.clone());
                result.push(sub_set);
            }
        }

        result
    }
}

// この方法では解けなかった
// DFSで解く
struct Solution2 {}
impl Solution2 {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut result = vec![];
        let mut set: HashSet<Vec<i32>> = HashSet::new();

        Self::dfs(&nums, 0, &mut current, &mut result, &mut set);

        result
    }

    fn dfs(
        nums: &Vec<i32>,
        start: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        set: &mut HashSet<Vec<i32>>,
    ) {
        /*
        current.sort();
        if !set.contains(current) {
            set.insert(current.clone());
            result.push(current.clone());
        }
        */
        result.push(current.clone());

        for i in start..(nums.len() as i32) {
            current.push(nums[i as usize]);
            Self::dfs(nums, i + 1, current, result, set);
            current.pop();
        }
    }
}

// AC
// C++の模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut current = vec![];
        let mut result = vec![];

        Self::dfs(&nums, 0, &mut current, &mut result);

        result
    }

    fn dfs(nums: &Vec<i32>, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());

        for i in start..nums.len() as i32 {
            if i > start && nums[i as usize] == nums[i as usize - 1] {
                continue;
            }

            current.push(nums[i as usize]);
            Self::dfs(nums, i + 1, current, result);
            current.pop();
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = vec![];
        Self::backtrack(0_usize, &mut result, &mut nums, &mut vec![]);

        result
    }

    fn backtrack(mut i: usize, result: &mut Vec<Vec<i32>>, nums: &Vec<i32>, subset: &mut Vec<i32>) {
        if i == nums.len() {
            result.push(subset.clone());
            return;
        }

        subset.push(nums[i]);
        Self::backtrack(i + 1, result, nums, subset);
        subset.pop();

        while i + 1 < nums.len() && nums[i] == nums[i + 1] {
            i += 1;
        }
        Self::backtrack(i + 1, result, nums, subset);
    }
}

fn main() {
    let case_1 = vec![1, 2, 2];
    let case_2 = vec![0];
    let case_3 = vec![4, 1, 0];

    println!("case_1: {:?}", Solution::subsets_with_dup(case_1.clone()));
    println!("case_2: {:?}", Solution::subsets_with_dup(case_2.clone()));
    println!("case_3: {:?}", Solution::subsets_with_dup(case_3.clone()));

    /*
    println!("case_1: {:?}", Solution2::subsets_with_dup(case_1.clone()));
    println!("case_2: {:?}", Solution2::subsets_with_dup(case_2.clone()));
    println!("case_3: {:?}", Solution2::subsets_with_dup(case_3.clone()));
    */

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::subsets_with_dup(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::subsets_with_dup(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::subsets_with_dup(case_3.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAns::subsets_with_dup(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::subsets_with_dup(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAns::subsets_with_dup(case_3.clone())
    );
}
