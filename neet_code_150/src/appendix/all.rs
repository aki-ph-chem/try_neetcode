fn main() {
    // all()は全ての要素がboolを返すクロージャーがtrueを返すならばtrueを返す

    // 全て偶数
    let nums = vec![2, 4, 6, 8];
    println!("nums: {}", nums.iter().all(|x| x % 2 == 0));

    // 偶数と奇数
    let nums = vec![2, 5, 6, 8, 9];
    println!("nums: {}", nums.iter().all(|x| x % 2 == 0));
}
