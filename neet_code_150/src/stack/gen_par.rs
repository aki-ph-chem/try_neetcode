struct Solution {}

// 解けなかった😭
// n: 1 ~ 8
impl Solution {
    pub fn gen_par(n: i32) -> Vec<String> {
        let mut result:Vec<String> = vec![];

        for i in 0..n {
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // Using the function stack instead of an explicitly allocated Vec
        let mut res: Vec<String> = vec![];

        fn backtrack(res: &mut Vec<String>, s: String, open: i32, close: i32) {
            if open == 0 && close == 0 {
                res.push(s);
                return;
            }
            if open == close {
                backtrack(res, s.clone() + "(", open - 1, close);
            } else {
                if open > 0 {
                    backtrack(res, s.clone() + "(", open - 1, close);
                }
                if close > 0 {
                    backtrack(res, s.clone() + ")", open, close - 1);
                }
            }
        }

        backtrack(&mut res, String::from(""), n, n);
        res
    }
}

fn main() {
    let case_1 = 3; // "[((()))","(()())","(())()","()(())","()()()"]
    let case_2 = 1; // "["()"]
                    //
    println!("case_1: {:?}", SolutionAns::generate_parenthesis(case_1));
    println!("case_2: {:?}", SolutionAns::generate_parenthesis(case_2));
}
