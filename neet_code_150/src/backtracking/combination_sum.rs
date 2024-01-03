// 解けなかった
struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut tmp = 0;
        let mut idx = 0;
        while tmp < target {}

        vec![vec![1]]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let (mut result, mut curr) = (vec![], vec![]);
        Self::dfs(&candidates, target, &mut result, &mut curr);

        result
    }

    fn dfs(candidates: &[i32], target: i32, result: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>) {
        // 現在のcurrの和がtargetになるなら終了
        let sum: i32 = curr.iter().sum();
        if sum == target {
            result.push(curr.to_owned());
            return;
        // sumがtargetを超えたら終了
        } else if sum > target {
            return;
        }

        // currの更新
        for (i, &c) in candidates.iter().enumerate() {
            curr.push(c);
            Self::dfs(&candidates[i..], target, result, curr);
            curr.pop();
        }
    }
}

// C++の模範解答より
struct SolutionCppAns {}
impl SolutionCppAns {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut result = vec![];
        Self::helper(&candidates, target, 0, &mut current, &mut result);

        result
    }

    fn helper(
        candidates: &Vec<i32>,
        target: i32,
        idx: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if idx >= candidates.len() || target < 0 {
            return;
        }

        if target == 0 {
            result.push(current.clone());
            return;
        }

        //idx番目を選ぶ場合
        //idx番目をcurrentにpush
        current.push(candidates[idx]);
        Self::helper(candidates, target - candidates[idx], idx, current, result);

        // idx番目を選ばない場合
        // 末尾の要素を削除
        current.pop();
        Self::helper(candidates, target, idx + 1, current, result);
    }
}

fn main() {
    let case_1 = (vec![2, 3, 6, 7], 7);
    // => [[2,2,3],[7]]
    let case_2 = (vec![2, 3, 5], 8);
    // => [[2,2,2,2], [2,3,3], [3,5]]
    let case_3 = (vec![2], 1);
    // => [[]]
    let case_4 = (vec![2, 3, 6, 7], 7);
    // => [[2,2,3],[7]]

    println!(
        "case_1: {:?}",
        SolutionAns::combination_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAns::combination_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAns::combination_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {:?}",
        SolutionAns::combination_sum(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {:?}",
        SolutionCppAns::combination_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionCppAns::combination_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionCppAns::combination_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {:?}",
        SolutionCppAns::combination_sum(case_4.0.clone(), case_4.1)
    );
}
