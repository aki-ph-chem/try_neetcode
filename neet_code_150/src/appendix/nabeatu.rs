// 数字に3が含まれているか否かの判定
fn is_contain_3(mut x: i32) -> bool {
    while x > 0 {
        if x % 10 == 3 {
            return true;
        }

        x /= 10;
    }

    false
}

// 1 ~ n の間で3の倍数もしくは3が含まれる数字のときに'aho'と出力
fn nabeatu_n(n: i32) {
    for i in 1..=n {
        print!("{}: ",i);
        if i % 3 == 0 {
            println!("aho");
        } else if is_contain_3(i) {
            println!("aho");
        } else {
            println!("");
        }
    }
}

fn main() {
    nabeatu_n(100);
}
