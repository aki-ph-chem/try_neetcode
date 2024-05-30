fn main() {
    let str_list = ["abc", "xyz", "vwx"];

    // join()で文字列の配列を連結して一つの文字列に変換
    // デリミタを空文字列 "" を用いた
    let str_concated = str_list.join("");
    println!("str_concated: {}", str_concated);

    // デリミタ "+" を用いた
    let str_concated = str_list.join("+");
    println!("str_concated: {}", str_concated);
}
