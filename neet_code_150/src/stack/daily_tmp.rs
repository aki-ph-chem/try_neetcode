struct Solution {}

impl Solution {
    // TLE
    pub fn daily_tmp_sq(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            let tmp_v = temperatures[i];
            for j in i + 1..temperatures.len() {
                if tmp_v < temperatures[j] {
                    res[i] = (j - i) as i32;
                    break;
                }
            }
        }
        res
    }

    // 解けなかった😭
    pub fn daily_tmp(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = vec![];

        for i in 0..temperatures.len() {
            stack.push(temperatures[i]);
        }

        res
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = vec![]; // (temp, index)

        for (i, val) in temperatures.iter().enumerate() {
            while !stack.is_empty() && *val > stack.last().unwrap().0 {
                let (_, stack_index) = stack.pop().unwrap();
                res[stack_index] = (i - stack_index) as i32;
            }

            stack.push((*val, i));
        }
        res
    }
}

fn main() {
    let case_1 = vec![73, 74, 75, 71, 69, 72, 76, 73]; // [1,1,4,2,1,1,0,0]
    let case_2 = vec![30, 40, 50, 60]; // [1,1,10]
    let case_3 = vec![30, 60, 90]; // [1,1,0]

    println!("case_1: {:?}", Solution::daily_tmp_sq(case_1.clone()));
    println!("case_2: {:?}", Solution::daily_tmp_sq(case_2.clone()));
    println!("case_3: {:?}", Solution::daily_tmp_sq(case_3.clone()));

    println!("case_1: {:?}", SolutionAns::daily_temperatures(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::daily_temperatures(case_2.clone()));
    println!("case_3: {:?}", SolutionAns::daily_temperatures(case_3.clone()));
}
