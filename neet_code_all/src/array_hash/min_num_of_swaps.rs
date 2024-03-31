use std::collections::VecDeque;

// 全く思いつかなかった
struct Solution {}
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        1
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn min_swaps(s: String) -> i32 {
        let mut extra_close = 0;
        let mut max_close = 0;

        for c in s.chars() {
            if c == '[' {
                extra_close -= 1;
            } else {
                extra_close += 1;
            }

            max_close = max_close.max(extra_close);
        }

        (max_close + 1) / 2
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn min_swaps(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut result = 0;
        let mut stack = VecDeque::new();
        stack.push_back(']');

        for i in 0..s.len() {
            if let Some(stack_top) = stack.back() {
                if s[i] == ']' {
                    if *stack_top == '[' {
                        stack.pop_back();
                    } else {
                        stack.push_back('[');
                        result += 1;
                    }
                } else {
                    stack.push_back('[');
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = "][][".to_string();
    // => 1
    let case_2 = "]]][[[".to_string();
    // => 2

    println!("case_1: {}", SolutionAns::min_swaps(case_1.clone()));
    println!("case_2: {}", SolutionAns::min_swaps(case_2.clone()));

    println!("case_1: {}", SolutionAnsCpp::min_swaps(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::min_swaps(case_2.clone()));
}
