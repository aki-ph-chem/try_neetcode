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

// 部分的別解
// 文字列のsplitも自前実装
struct Solution2;
impl Solution2 {
    fn split_by(src: String) -> Vec<String> {
        let mut result = vec![];
        let src = src.chars().collect::<Vec<char>>();

        let mut i_1 = 0;
        while i_1 < src.len() {
            let mut tmp = "".to_string();
            if src[i_1] == '/' {
                let mut i_2 = i_1 + 1;
                while i_2 < src.len() && src[i_2] != '/' {
                    tmp.push(src[i_2]);
                    i_2 += 1;
                }
                result.push(tmp);
                i_1 = i_2 - 1;
            }

            i_1 += 1;
        }

        result
    }

    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];

        let tmp = Self::split_by(path);
        for i in &tmp {
            match i.as_str() {
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                "." | "" => continue,
                _ => stack.push(i),
            };
        }
        let mut result = "".to_string();
        for s in stack {
            result.push_str("/");
            result.push_str(&s);
        }
        if result.is_empty() {
            result.push('/');
        }

        result
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

    println!("case_1: {}", Solution2::simplify_path(case_1.clone()));
    println!("case_2: {}", Solution2::simplify_path(case_2.clone()));
    println!("case_3: {}", Solution2::simplify_path(case_3.clone()));
    println!("case_4: {}", Solution2::simplify_path(case_4.clone()));
}
