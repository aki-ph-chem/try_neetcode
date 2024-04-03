fn main() {
    // OR => 最下位のビットを1に
    let mut num = 0b11010_u32;
    num |= 1;
    println!("num = {:032b}", num);

    // AND => 最下位医ビットを取り出す
    let mut num = 0b11001_u32;
    num &= 1;
    println!("num = {:032b}", num);

    // 奇数の場合
    let mut num = 0b11001_u32;
    println!("num = {}", num);
    num ^= 1;
    println!("num = {}", num);

    // 偶数の場合
    let mut num = 0b11010_u32;
    println!("num = {}", num);
    num ^= 1;
    println!("num = {}", num);

    // numが2のべき乗か否かの判定
    let num = 8;
    if num & (num - 1) == 0 {
        println!("num = {num} is power of 2");
    } else {
        println!("num = {num} is not power of 2");
    }
    let num = 9;
    if num & (num - 1) == 0 {
        println!("num = {num} is power of 2");
    } else {
        println!("num = {num} is not power of 2");
    }
}
