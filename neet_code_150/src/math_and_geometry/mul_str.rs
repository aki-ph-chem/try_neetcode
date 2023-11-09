struct Solution {}
impl Solution {
    // 普通に計算するとオーバーフローする
    // しかし、多倍長整数を使うのは禁止
    pub fn multiply(num1: String, nums: String) -> String {
        "".to_string()
    }

    // 多分オーバーフローする
    pub fn multiply_b(num1: String, num2: String) -> String {
        let num_1 = num1.parse::<i64>().unwrap();
        let num_2 = num2.parse::<i64>().unwrap();

        format!("{}", num_1 * num_2)
    }
}

fn main() {
    let case_1 = ("2".to_string(), "3".to_string());
    let case_2 = ("123".to_string(), "456".to_string());

    println!("case_1: {}", Solution::multiply_b(case_1.0.clone(), case_1.1.clone()));
    println!("case_2: {}", Solution::multiply_b(case_2.0.clone(), case_2.1.clone()));
}
