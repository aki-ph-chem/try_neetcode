struct Solution {}

// n: 1 ~ 8
impl Solution {
    pub fn gen_par(n: i32) -> Vec<String> {
        let mut result:Vec<String> = vec![];

        for i in 0..n {
        }

        result
    }
}

// '(',')'の判定
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


fn main() {
    let case_1 = 3; // "[((()))","(()())","(())()","()(())","()()()"]
    let case_2 = 1; // "["()"]
}
