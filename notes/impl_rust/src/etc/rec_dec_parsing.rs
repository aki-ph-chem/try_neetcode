// 再帰下降構文解析で四則演算を書いてみる
// 参考: https://dai1741.github.io/maximum-algo-2012/docs/parsing/
///
/// 構文を表すEBNF
/// <expr>   ::= <term> [ ('+'|'-') <term> ]*
/// <term>   ::= <factor> [ ('*'|'/') <factor> ]*
/// <factor> ::= <number> | '(' <expr> ')'
/// <number> :== 1つ以上の数字

// 足し算、引き算
pub fn expr(s: &[u8], i: &mut usize) -> i32 {
    let mut val = term(s, i);
    while *i < s.len() && (s[*i] == b'+' || s[*i] == b'-') {
        let op = s[*i];
        *i += 1;

        let val_2 = term(s, i);
        if op == b'+' {
            val += val_2;
        } else {
            val -= val_2;
        }
    }

    val
}

// 掛け算, 割り算
pub fn term(s: &[u8], i: &mut usize) -> i32 {
    let mut val = factor(s, i);
    while *i < s.len() && (s[*i] == b'*' || s[*i] == b'/') {
        let op = s[*i];
        *i += 1;

        let val_2 = factor(s, i);
        if op == b'*' {
            val *= val_2;
        } else {
            val /= val_2;
        }
    }

    val
}

// 括弧()
pub fn factor(s: &[u8], i: &mut usize) -> i32 {
    if (s[*i] as char).is_numeric() {
        return number(s, i);
    }

    *i += 1;
    let res = expr(s, i);
    *i += 1;

    res
}

// 数字
pub fn number(s: &[u8], i: &mut usize) -> i32 {
    let mut n = s[*i] - b'0';
    *i += 1;

    while *i < s.len() && (s[*i] as char).is_numeric() {
        n = n * 10 + s[*i] - b'0';
        *i += 1;
    }

    n as i32
}

fn main() {
    let expr_0 = "1+2*6/(10-7)".as_bytes();
    // => 5

    let mut i = 0;
    println!("expr = {}", expr(&expr_0, &mut i));
}
