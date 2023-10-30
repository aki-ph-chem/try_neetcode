struct Solution {}

// AC
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut numbers = vec![];
        for c in tokens {
            match c.parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(_e) => {
                    let a = match numbers.pop() {
                        Some(num) => num,
                        None => {
                            panic!("error");
                        }
                    };
                    let b = match numbers.pop() {
                        Some(num) => num,
                        None => {
                            panic!("error");
                        }
                    };
                    match c.as_str() {
                        "+" => {
                            let result = b + a;
                            numbers.push(result);
                        }
                        "-" => {
                            let result = b - a;
                            numbers.push(result);
                        }
                        "*" => {
                            let result = b * a;
                            numbers.push(result);
                        }
                        "/" => {
                            let result = b / a;
                            numbers.push(result);
                        }
                        _ => {
                            panic!("invalid operator");
                        }
                    }
                }
            }
        }
        println!("{}", numbers.iter().rev().next().unwrap());
        match numbers.iter().rev().next() {
            Some(num) => *num,
            None => {
                panic!("error");
            }
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match &token[..] {
                "+" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand + second_operand)
                }
                "-" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand - second_operand)
                }
                "*" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand * second_operand)
                }
                "/" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand / second_operand)
                }
                value => stack.push(value.parse::<i32>().unwrap()),
            }
        }

        println!("{}", stack[0]);
        stack[0]
    }
}

fn main() {
    let case_1: Vec<String> = vec!["4", "13", "5", "/", "+"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let case_2: Vec<String> = vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    Solution::eval_rpn(case_1.clone());
    Solution::eval_rpn(case_2.clone());

    SolutionAns::eval_rpn(case_1.clone());
    SolutionAns::eval_rpn(case_2.clone());
}
