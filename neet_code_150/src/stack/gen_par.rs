struct Solution {}

// 解けなかった😭
// n: 1 ~ 8
impl Solution {
    pub fn gen_par(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for i in 0..n {}

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // Using the function stack instead of an explicitly allocated Vec
        let mut res: Vec<String> = vec![];

        // open, closeを両方ともnからスタートして0に等しくなるまで減らしながら再帰
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

// C++の模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        Self::generate(n, 0, 0, "".to_string(), &mut res);

        res
    }

    // open, closeを両方とも0からスタートしてnに等しくなるまで増やしながら再帰
    fn generate(n: i32, open: i32, close: i32, str: String, result: &mut Vec<String>) {
        if open == n && close == n {
            result.push(str);
            return;
        }

        if open < n {
            Self::generate(n, open + 1, close, str.clone() + "(", result);
        }
        if open > close {
            Self::generate(n, open, close + 1, str.clone() + ")", result);
        }
    }
}

fn main() {
    let case_1 = 3; // "[((()))","(()())","(())()","()(())","()()()"]
    let case_2 = 1; // "["()"]
                    //
    println!("case_1: {:?}", SolutionAns::generate_parenthesis(case_1));
    println!("case_2: {:?}", SolutionAns::generate_parenthesis(case_2));

    println!("case_1: {:?}", SolutionAnsCpp::generate_parenthesis(case_1));
    println!("case_2: {:?}", SolutionAnsCpp::generate_parenthesis(case_2));
}
