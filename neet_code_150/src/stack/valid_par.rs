use std::collections::HashMap;

#[derive(Debug)]
struct Solution {}

impl Solution {
    // だめ: case_4に対応できない
    pub fn is_valid(s: String) -> bool {
        let mut maru = vec![];
        let mut nami = vec![];
        let mut kaku = vec![];

        for c in s.chars() {
            match c {
                '(' => maru.push(c),
                '{' => nami.push(c),
                '[' => kaku.push(c),
                ')' => match maru.pop() {
                    Some(_t) => {}
                    None => {
                        return false;
                    }
                },
                '}' => match nami.pop() {
                    Some(_t) => {}
                    None => {
                        return false;
                    }
                },
                ']' => match kaku.pop() {
                    Some(_t) => {}
                    None => {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
        }

        if !maru.is_empty() {
            return false;
        }

        if !nami.is_empty() {
            return false;
        }

        if !kaku.is_empty() {
            return false;
        }

        true
    }

    // AC
    pub fn is_valid_2(s: String) -> bool {
        let mut state = vec![];

        for c in s.chars() {
            match c {
                '(' => state.push(c),
                '{' => state.push(c),
                '[' => state.push(c),
                ')' => match state.pop() {
                    Some(t) => {
                        if t != '(' {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                '}' => match state.pop() {
                    Some(t) => {
                        if t != '{' {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                ']' => match state.pop() {
                    Some(t) => {
                        if t != '[' {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
        }
        if !state.is_empty() {
            return false;
        }

        true
    }
}

// '(',')'のみ場合の判定
pub fn is_valid_maru(s: &str) -> bool {
    let mut state = vec![];

    for c in s.chars() {
        if c == '(' {
            state.push(c);
        } else {
            match state.pop() {
                Some(t) => {
                    if c != t {
                        return false;
                    }
                }
                None => {
                    println!("Error");
                    return false;
                }
            }
        }
    }

    if !state.is_empty() {
        return false;
    }

    true
}

struct SolutionAns {}
impl SolutionAns {
    // 模範解答
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let opening = HashMap::from([(']', '['), (')', '('), ('}', '{')]);

        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                _ => {
                    if stack.iter().last() == opening.get(&c) {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

// 後で時間を置いて解いたときの解
struct SolutionLatter;
impl SolutionLatter {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            if let Some(stack_top) = stack.last() {
                match (stack_top, c) {
                    ('{', '}') => {
                        stack.pop();
                    }
                    ('(', ')') => {
                        stack.pop();
                    }
                    ('[', ']') => {
                        stack.pop();
                    }
                    _ => {
                        stack.push(c);
                    }
                }
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}

fn main() {
    let case_1 = "()".to_string();
    let case_2 = "()[]{}".to_string();
    let case_3 = "(]".to_string();
    let case_4 = "([)]".to_string();

    println!("case_1: {}", Solution::is_valid(case_1.clone())); // true
    println!("case_2: {}", Solution::is_valid(case_2.clone())); // true
    println!("case_3: {}", Solution::is_valid(case_3.clone())); // false
    println!("case_4: {}", Solution::is_valid(case_4.clone())); // false

    println!("case_1: {}", Solution::is_valid_2(case_1.clone())); // true
    println!("case_2: {}", Solution::is_valid_2(case_2.clone())); // true
    println!("case_3: {}", Solution::is_valid_2(case_3.clone())); // false
                                                                  // case_4の答えがおかしい
    println!("case_4: {}", Solution::is_valid_2(case_4.clone())); // false

    println!("case_1: {}", SolutionAns::is_valid(case_1.clone())); // true
    println!("case_2: {}", SolutionAns::is_valid(case_2.clone())); // true
    println!("case_3: {}", SolutionAns::is_valid(case_3.clone())); // false
    println!("case_4: {}", SolutionAns::is_valid(case_4.clone())); // false

    /*
    println!("()(): {}", is_valid_maru("()()"));
    println!("()): {}", is_valid_maru("())"));
    println!("()(: {}", is_valid_maru("()("));
    println!("(()): {}", is_valid_maru("(())"));
    println!("(()()): {}", is_valid_maru("(()())"));
    println!("())()): {}", is_valid_maru("())())"));
    */

    println!("case_1: {}", SolutionLatter::is_valid(case_1.clone())); // true
    println!("case_2: {}", SolutionLatter::is_valid(case_2.clone())); // true
    println!("case_3: {}", SolutionLatter::is_valid(case_3.clone())); // false
    println!("case_4: {}", SolutionLatter::is_valid(case_4.clone())); // false
}
