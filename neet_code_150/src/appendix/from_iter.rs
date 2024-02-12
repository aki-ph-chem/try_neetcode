use std::iter::FromIterator;

fn main() {
    // 奇数のベクタを計算
    let odd_array: Vec<i32> = (0..5).map(|i| 2 * i + 1).collect();
    println!("odd_array: {:?}", odd_array);
    let odd_array = Vec::from_iter((5..10).map(|x| 2 * x + 1));
    println!("odd_array: {:?}", odd_array);
}
