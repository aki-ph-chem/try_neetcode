// AC
// C++の模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut current = vec![];
        let mut result = vec![];

        Self::dfs(&candidates, target, 0, 0, &mut current, &mut result);

        result
    }

    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        sum: i32,
        start: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum > target {
            return;
        }

        if sum == target {
            result.push(current.clone());
        }

        for i in start..candidates.len() as i32 {
            if i > start && candidates[i as usize] == candidates[i as usize - 1] {
                continue;
            }

            current.push(candidates[i as usize]);
            Self::dfs(
                candidates,
                target,
                sum + candidates[i as usize],
                i + 1,
                current,
                result,
            );
            current.pop();
        }
    }
}

fn main() {
    let case_1 = (vec![10, 1, 2, 7, 6, 1, 5], 8);
    let case_2 = (vec![2, 5, 2, 1, 2], 5);

    println!(
        "case_1: {:?}",
        SolutionAns::combination_sum2(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAns::combination_sum2(case_2.0.clone(), case_2.1)
    );
}
