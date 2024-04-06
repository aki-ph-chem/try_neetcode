use std::collections::VecDeque;

struct Solution {}
impl Solution {
    // AC
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = vec![];

        for op in &operations {
            match op.as_str() {
                "C" => {
                    stack.pop();
                }
                "D" => {
                    let d_top = 2 * stack.last().unwrap();
                    stack.push(d_top)
                }
                "+" => {
                    let mut sum = 0;
                    sum += stack.iter().rev().nth(0).unwrap();
                    sum += stack.iter().rev().nth(1).unwrap();
                    stack.push(sum);
                }
                _ => {
                    let num = op.parse::<i32>().unwrap();
                    stack.push(num);
                }
            }
        }

        stack.iter().fold(0, |acc, x| acc + x)
    }

    // AC
    pub fn cal_points_2(operations: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();

        for op in &operations {
            match op.as_str() {
                "C" => {
                    stack.pop_back();
                }
                "D" => {
                    let d_top = 2 * stack.back().unwrap();
                    stack.push_back(d_top)
                }
                "+" => {
                    let top_prev = stack.pop_back().unwrap();
                    let sum = top_prev + stack.back().unwrap();
                    stack.push_back(top_prev);
                    stack.push_back(sum);
                }
                _ => {
                    let num = op.parse::<i32>().unwrap();
                    stack.push_back(num);
                }
            }
        }

        stack.iter().fold(0, |acc, x| acc + x)
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn cal_points(operatoins: Vec<String>) -> i32 {
        let mut points = vec![];

        for op in &operatoins {
            match op.as_str() {
                "D" => {
                    let new_point = 2 * points.last().unwrap();
                    points.push(new_point);
                }
                "C" => {
                    points.pop();
                }
                "+" => {
                    let len = points.len();
                    points.push(points[len - 1] + points[len - 2]);
                }
                _ => {
                    points.push(op.parse::<i32>().unwrap());
                }
            }
        }

        points.iter().sum()
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = VecDeque::new();
        let mut result = 0;

        for op in &operations {
            match op.as_str() {
                "+" => {
                    let first = stack.pop_back().unwrap();
                    let second = *stack.back().unwrap();
                    stack.push_back(first);
                    stack.push_back(first + second);
                    result += first + second;
                }
                "D" => {
                    result += 2 * stack.back().unwrap();
                    stack.push_back(2 * stack.back().unwrap());
                }
                "C" => {
                    result -= stack.back().unwrap();
                    stack.pop_back();
                }
                _ => {
                    let num = op.parse::<i32>().unwrap();
                    result += num;
                    stack.push_back(num);
                }
            }
        }

        result
    }
}

fn main() {
    let case_1: Vec<String> = vec!["5", "2", "C", "D", "+"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    // => 30
    let case_2: Vec<String> = vec!["5", "-2", "4", "C", "D", "9", "+", "+"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    // => 27
    let case_3: Vec<String> = vec!["1", "C"].iter().map(|x| x.to_string()).collect();
    // => 0

    println!("case_1: {}", Solution::cal_points(case_1.clone()));
    println!("case_2: {}", Solution::cal_points(case_2.clone()));
    println!("case_3: {}", Solution::cal_points(case_3.clone()));

    println!("case_1: {}", Solution::cal_points_2(case_1.clone()));
    println!("case_2: {}", Solution::cal_points_2(case_2.clone()));
    println!("case_3: {}", Solution::cal_points_2(case_3.clone()));

    println!("case_1: {}", SolutionAns::cal_points(case_1.clone()));
    println!("case_2: {}", SolutionAns::cal_points(case_2.clone()));
    println!("case_3: {}", SolutionAns::cal_points(case_3.clone()));

    println!("case_1: {}", SolutionAnsCpp::cal_points(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::cal_points(case_2.clone()));
    println!("case_3: {}", SolutionAnsCpp::cal_points(case_3.clone()));
}
