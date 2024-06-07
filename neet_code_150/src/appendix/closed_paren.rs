// カッコ'()'が閉じているか否か判定する
// stackの問題の典型例
pub fn valid_paren(p_str: String) -> bool {
    let mut stack = vec![];
    for c in p_str.chars() {
        if let Some(stack_top) = stack.last() {
            match (stack_top, c) {
                ('(', ')') => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

fn main() {
    let case_1 = "(())".to_string();
    // => true
    let case_2 = "()()()".to_string();
    // => true
    let case_3 = ")(".to_string();
    // => false
    let case_4 = "(()()".to_string();
    // => false

    println!("case_1: {}", valid_paren(case_1.clone()));
    println!("case_2: {}", valid_paren(case_2.clone()));
    println!("case_3: {}", valid_paren(case_3.clone()));
    println!("case_4: {}", valid_paren(case_4.clone()));
}
