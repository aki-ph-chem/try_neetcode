// '..'の解析の時点でわからなくなった('...'は尚更😅)
struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut result = vec![];
        for c in path.chars() {
            if let Some(result_last) = result.last() {
                match (*result_last, c) {
                    ('/', _) => {
                        if c != '/' {
                            result.push(c);
                        }
                    }
                    ('.', _) => {
                        if c == '.' {
                            result.pop();
                        } else {
                            result.pop();
                            result.push(c);
                        }
                    }
                    (_, _) => {
                        result.push(c);
                    }
                }
            } else {
                result.push(c);
            }
        }

        if let Some(result_last) = result.last() {
            if result.len() > 1 && *result_last == '/' {
                result.pop();
            }
        }

        result.iter().collect::<String>()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];

        for i in path.split("/") {
            match i {
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                "." | "" => continue,
                _ => stack.push(i),
            };
        }

        format!("/{}", stack.join("/"))
    }
}

fn main() {
    let case_1 = "/home/".to_string();
    let case_2 = "/home//foo".to_string();
    let case_3 = "/../".to_string();
    let case_4 = "/.../a/../b/c/../d/./".to_string();

    /*
    println!("case_1: {}", Solution::simplify_path(case_1.clone()));
    println!("case_2: {}", Solution::simplify_path(case_2.clone()));
    println!("case_3: {}", Solution::simplify_path(case_3.clone()));
    println!("case_4: {}", Solution::simplify_path(case_4.clone()));
    */

    println!("case_1: {}", SolutionAns::simplify_path(case_1.clone()));
    println!("case_2: {}", SolutionAns::simplify_path(case_2.clone()));
    println!("case_3: {}", SolutionAns::simplify_path(case_3.clone()));
    println!("case_4: {}", SolutionAns::simplify_path(case_4.clone()));
}
