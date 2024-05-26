// 模範解答(Python)は全く同じだった
struct Solution;
impl Solution {
    // AC
    pub fn remove_stars(s: String) -> String {
        let mut result = vec![];
        for c in s.chars() {
            if c == '*' {
                result.pop();
            } else {
                result.push(c);
            }
        }

        result.iter().collect::<String>()
    }
}

fn main() {
    let case_1 = "leet**cod*e".to_string();
    // => "lecoe"
    let case_2 = "erase*****".to_string();
    // => ""

    println!("case_1: {}", Solution::remove_stars(case_1.clone()));
    println!("case_2: {}", Solution::remove_stars(case_2.clone()));
}
