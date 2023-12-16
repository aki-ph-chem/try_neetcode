// C++の模範解答より
struct Solution {}
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let n = s.len();
        let mut vec_char:Vec<char> = s.chars().collect();

        // 文字列の頭からbalancedを'(' or '*'ならインクリメント
        // それ以外ならデクリメント
        let mut blanced = 0;
        for i in 0..n {
            if vec_char[i] == '(' || vec_char[i] == '*' {
                blanced += 1;
            } else {
                blanced -= 1;
            }

            if blanced < 0 {
                return false;
            }
        }

        // バランスが取れていればok
        if blanced == 0 {
            return true;
        }

        // 文字列の後ろからbalanceを')'or'*'ならばインクリメント、それ以外ならばデクリメント 
        blanced = 0;
        for i in (0..n).rev() {
            if vec_char[i] == ')' || vec_char[i] == '*' {
                blanced += 1;
            } else {
                blanced -= 1;
            }

            if blanced < 0 {
                return false;
            }
        } 

        true
    }
}

fn main() {
    let case_1 = "()()".to_string(); // true
    let case_2 = "())".to_string(); // false
    let case_3 = "*)".to_string(); // true
    let case_4 = "(*".to_string(); // true

    println!("case_1: {}", Solution::check_valid_string(case_1));
    println!("case_1: {}", Solution::check_valid_string(case_2));
    println!("case_1: {}", Solution::check_valid_string(case_3));
    println!("case_1: {}", Solution::check_valid_string(case_4));
}
