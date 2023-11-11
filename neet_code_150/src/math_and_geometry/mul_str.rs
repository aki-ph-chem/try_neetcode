struct Solution {}
impl Solution {
    // しかし、多倍長整数ライブラリを使うのは禁止
    // 筆算を実装する
    // C++の模範解答をRustに変換
    // AC
    pub fn multiply(num1: String, num2: String) -> String {
        let num1_v: Vec<char> = num1.chars().collect();
        let num2_v: Vec<char> = num2.chars().collect();

        let l_num1 = num1_v.len();
        let l_num2 = num2_v.len();

        let mut result = vec!['0'; l_num1 + l_num2];

        for i in (0..l_num1).rev() {
            for j in (0..l_num2).rev() {
                let sum = (num1_v[i] as u8 - '0' as u8) * (num2_v[j] as u8 - '0' as u8)
                    + (result[i + j + 1] as u8 - '0' as u8);
                result[i + j + 1] = (sum % 10 + '0' as u8) as char;
                result[i + j] = ( result[i + j] as u8  + sum / 10) as char;
            }
        }

        for i in 0..result.len() {
            if result[i] != '0' {
                return result[i..].iter().collect();
            }
        }

        "0".to_string()
    }

    // 多分オーバーフローする
    // オーバーフローした
    pub fn multiply_b(num1: String, num2: String) -> String {
        let num_1 = num1.parse::<i64>().unwrap();
        let num_2 = num2.parse::<i64>().unwrap();

        format!("{}", num_1 * num_2)
    }
}

fn main() {
    let case_1 = ("2".to_string(), "3".to_string());
    let case_2 = ("123".to_string(), "456".to_string());

    println!(
        "case_1: {}",
        Solution::multiply_b(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::multiply_b(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        Solution::multiply(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::multiply(case_2.0.clone(), case_2.1.clone())
    );
}
