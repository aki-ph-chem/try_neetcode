// 解けなかった
struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        //let mut nums = vec![];
        //let mut part = vec![];

        let mut i_1 = 0;
        while i_1 < s.len() {}

        let mut result = vec!['a'];

        result.iter().collect::<String>()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];

        for c in s.chars() {
            if c != ']' {
                stack.push(c.to_string());
            } else {
                let mut sub_str = String::new();
                while stack.last() != Some(&"[".to_string()) {
                    sub_str = stack.pop().unwrap().to_string() + &sub_str;
                }
                stack.pop();

                let mut multi_plier = String::new();
                while !stack.is_empty() && stack.last().unwrap().parse::<i32>().is_ok() {
                    multi_plier = stack.pop().unwrap().to_string() + &multi_plier;
                }
                stack.push(sub_str.repeat(multi_plier.parse::<usize>().unwrap()));
            }
        }

        stack.join("")
    }

    // AC
    // while letを使って書く
    pub fn decode_string_2(s: String) -> String {
        let mut stack = vec![];

        for c in s.chars() {
            if c != ']' {
                stack.push(c.to_string());
            } else {
                let mut sub_str = String::new();
                while let Some(stack_last) = stack.last() {
                    if stack_last != "[" {
                        sub_str = stack_last.to_string() + &sub_str;
                        stack.pop();
                    } else {
                        break;
                    }
                }
                stack.pop();

                let mut multi_plier = String::new();
                while let Some(stack_last) = stack.last() {
                    match stack_last.parse::<i32>() {
                        Ok(_n) => {
                            multi_plier = stack_last.to_string() + &multi_plier;
                            stack.pop();
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }

                stack.push(sub_str.repeat(multi_plier.parse::<usize>().unwrap()));
            }
        }

        stack.join("")
    }
}
fn main() {
    let case_1 = "3[a]2[bc]".to_string();
    // => aaabcbcbc
    let case_2 = "3[a2[c]]".to_string();
    // => accaccacc

    println!("case_1: {}", SolutionAns::decode_string(case_1.clone()));
    println!("case_2: {}", SolutionAns::decode_string(case_2.clone()));

    println!("case_1: {}", SolutionAns::decode_string_2(case_1.clone()));
    println!("case_2: {}", SolutionAns::decode_string_2(case_2.clone()));
}
