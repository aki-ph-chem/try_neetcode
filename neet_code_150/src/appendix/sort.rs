fn main() {
    // 普通のsort()
    let mut nums = vec![1, 5, 3, 7, 10, 2];
    nums.sort();
    println!("nums: {:?}", nums);

    // sort_by()で文字列の長さ順でsort
    // sort_by()の引数には配列の型がTのときT&の引数が二個でstd::cmp::Orderingを返すクロージャー
    let mut strings = vec!["ffoog", "ho", "a", "hogex_xhoge", "abcd", "xyz"];
    strings.sort_by(|x, y| x.len().cmp(&y.len()));
    println!("string: {:?}", strings);
}
