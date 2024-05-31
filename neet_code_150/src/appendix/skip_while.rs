fn main() {
    // skip_while()
    // 最初に条件を満たす場合にスキップ

    let array_char = "00200".chars().collect::<Vec<char>>();
    let result = array_char
        .iter()
        .skip_while(|&c| *c == '0')
        .collect::<String>();
    println!("result : {}", result);

    let array_char = "00120302200".chars().collect::<Vec<char>>();
    let result = array_char
        .iter()
        .skip_while(|&c| *c == '0')
        .collect::<String>();

    println!("result : {}", result);
}
