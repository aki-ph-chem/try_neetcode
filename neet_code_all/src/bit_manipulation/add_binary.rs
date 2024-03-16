// 解けなかった
struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result: Vec<char> = vec![];
        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();

        let mut i = 0;
        while let Some(c_a) = a.pop() {
            while i < b.len() {
                let digit = (c_a as u8 - b'0') ^ (b[i] as u8 - b'0');
                let carry = (c_a as u8 - b'0') & (b[i] as u8 - b'0');

                result.push((digit + b'0') as char);
                i += 1;
            }
        }

        result.iter().collect()
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = String::new();
        let mut carry = 0;

        let (mut a, mut b) = (a.as_bytes().to_owned(), b.as_bytes().to_owned());
        a.reverse();
        b.reverse();

        for i in 0..a.len().max(b.len()) {
            let digit_a = if i < a.len() { a[i] - b'0' } else { 0 };
            let digit_b = if i < b.len() { b[i] - b'0' } else { 0 };

            let total = digit_a + digit_b + carry;
            let c = (total % 2).to_string();
            res = c + &res;
            carry = total / 2;
        }

        if carry != 0 {
            res = "1".to_string() + &res;
        }

        res
    }
}

fn main() {
    let case_1 = ("11".to_string(), "1".to_string());
    // "100"
    let case_2 = ("1010".to_string(), "1011".to_string());
    // "10101"

    /*
    println!(
        "case_1: {}",
        Solution::add_binary(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::add_binary(case_2.0.clone(), case_2.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::add_binary(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::add_binary(case_2.0.clone(), case_2.1.clone())
    );
}
