fn main() {
    let s = "abc";
    let mut stack = "".to_string();
    // 文字列sを三回繰り返しす(連結する)
    stack.push_str(&s.repeat(3));
    println!("stack: {}", stack);

    let s = "xyz";
    let mut stack = vec![];
    // 文字列sを三回繰り返し(連結する)てできた文字列をベクタにプッシュする
    stack.push(s.repeat(3));
    println!("stack: {:?}", stack);
}
