struct Solution {}
impl Solution {
    //AC
    pub fn min_const_climbing_stairs(cost: Vec<i32>) -> i32 {
        // 0 スタート
        let mut table_0 = vec![1 << 30; cost.len() + 1];
        // 1 スタート
        let mut table_1 = vec![1 << 30; cost.len() + 1];

        table_0[0] = 0;
        table_1[0] = 0;

        for i in 1..(cost.len() + 1) {
            if i == 1 {
                // 0 スタートならば、1に来るコストは cost[0]
                table_0[i] = cost[0];
                // 1 スタートならば、1に来るコストは 0
                table_1[i] = 0;
            } else {
                table_0[i] =
                    std::cmp::min(table_0[i - 1] + cost[i - 1], table_0[i - 2] + cost[i - 2]);
                table_1[i] =
                    std::cmp::min(table_1[i - 1] + cost[i - 1], table_1[i - 2] + cost[i - 2]);
            }
        }

        std::cmp::min(table_0[cost.len()], table_1[cost.len()])
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        for i in 2..cost.len(){
            cost[i] += cost[i- 1].min(cost[i-2]);
        }

        let len = cost.len();

        cost[len - 1].min(cost[len - 2])
    }
}

fn main() {
    let case_1 = vec![10, 15, 20];
    let case_2 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];

    println!(
        "case_1: {}",
        Solution::min_const_climbing_stairs(case_1.clone())
    );

    println!(
        "case_2: {}",
        Solution::min_const_climbing_stairs(case_2.clone())
    );
}
