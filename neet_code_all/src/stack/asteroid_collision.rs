// asteroids[i]: 絶対値は惑星のサイズ、符号は進行方向(+右、-左)
// 小さい惑星と大きい惑星が出会うと消滅する
use std::cmp::Ordering;

// 解けなかった
struct Solution;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for a in asteroids {
            if result.is_empty() {
                result.push(a);
            } else {
                while let Some(res_last) = result.last() {
                    if *res_last < 0 || (*res_last > 0 && a > 0) {
                        result.push(a);
                        break;
                    } else {
                        if *res_last == a.abs() {
                            result.pop();
                            break;
                        }

                        if *res_last < a.abs() {
                            result.pop();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        result
    }

    pub fn asteroid_collision_2(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for a in asteroids {
            if result.is_empty() {
                result.push(a);
            } else {
                if let Some(res_last) = result.last() {
                    if *res_last < 0 || (*res_last > 0 && a > 0) {
                        result.push(a);
                    } else {
                        if *res_last == a.abs() {}

                        while *res_last < a.abs() {}
                    }
                }
            }
            println!("a, result: {},{:?}", a, result);
        }

        result
    }
}

// 模範解答(C++のものと基本的に方針は同じ(Oderingの場所以外))
struct SolutionAns;
impl SolutionAns {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];

        for mut ast in asteroids {
            while !stack.is_empty() && ast < 0 && stack.last() > Some(&0) {
                let diff = ast + stack.last().unwrap();
                match diff.cmp(&0) {
                    Ordering::Less => {
                        stack.pop();
                    }
                    Ordering::Greater => {
                        ast = 0;
                    }
                    Ordering::Equal => {
                        ast = 0;
                        stack.pop();
                    }
                }
            }
            if ast != 0 {
                stack.push(ast);
            }
        }

        stack
    }
}

fn main() {
    let case_1 = vec![5, 10, -5];
    // => [5, 10]
    let case_2 = vec![8, -8];
    // => []
    let case_3 = vec![10, 2, -5];
    // => [10]
    let case_4 = vec![-2, -1, 1, 2];
    // => [-2,-1,1,2]
    let case_5 = vec![1, -2, -2, -2];
    // => [-2,-2]

    /*
    println!("case_1: {:?}", Solution::asteroid_collision(case_1.clone()));
    println!("case_2: {:?}", Solution::asteroid_collision(case_2.clone()));
    println!("case_3: {:?}", Solution::asteroid_collision(case_3.clone()));

    println!("case_4: {:?}", Solution::asteroid_collision(case_4.clone()));
    */
    /*
    println!(
        "case_1: {:?}",
        Solution::asteroid_collision_2(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::asteroid_collision_2(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        Solution::asteroid_collision_2(case_3.clone())
    );
    println!(
        "case_4: {:?}",
        Solution::asteroid_collision_2(case_4.clone())
    );
    println!(
        "case_5: {:?}",
        Solution::asteroid_collision_2(case_5.clone())
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAns::asteroid_collision(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::asteroid_collision(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAns::asteroid_collision(case_3.clone())
    );
    println!(
        "case_4: {:?}",
        SolutionAns::asteroid_collision(case_4.clone())
    );
}
