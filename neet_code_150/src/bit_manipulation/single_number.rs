use std::collections::HashMap;

// 縛り: O(N)で実装し、コレクション型の値は使わないこと
struct Solution {}
impl Solution {
    // 解けなかった
    pub fn single_number(nums: Vec<i32>) -> i32 {
        2
    }

    // 縛り無視ならこう実装する
    // AC
    pub fn single_number_set(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for v in &nums {
            *map.entry(*v).or_default() += 1;
        }

        let mut result = 0;
        for (v, n) in map {
            if n == 1 {
                result = v;
                break;
            }
        } 

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n;
        }
        res
    }
}

// ex) [2, 2, 1]
// 2 ^ 2 ^ 1
// = 0 ^ 1
// = 1

// ex) [4, 1, 2, 1, 2]
// 4 ^ 1 ^ 2 ^ 1 ^ 2
// = 4 ^ 3 ^ 3
// = 4

fn main() {
    let case_1 = vec![2, 2, 1];
    let case_2 = vec![4, 1, 2, 1, 2];
    let case_3 = vec![1];

    println!("case_1: {}", Solution::single_number_set(case_1.clone()));
    println!("case_2: {}", Solution::single_number_set(case_2.clone()));
    println!("case_3: {}", Solution::single_number_set(case_3.clone()));

    println!("case_1: {}", SolutionAns::single_number(case_1.clone()));
    println!("case_2: {}", SolutionAns::single_number(case_2.clone()));
    println!("case_3: {}", SolutionAns::single_number(case_3.clone()));
}
