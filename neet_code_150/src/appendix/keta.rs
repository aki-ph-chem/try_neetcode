// 下位の桁から桁の値を取得する
fn get_digit_back(mut n: i32) -> Vec<i32> {
    let mut result = vec![];
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }

    result
}

// 上位の桁から桁の値を取得する
fn get_digit_front(n: i32) -> Vec<i32> {
    let mut n = n as i64;
    let mut div = 1_i64;
    while n >= 10 * div {
        div *= 10;
    }

    let mut result = vec![];
    while n != 0 {
        result.push((n / div) as i32);
        n %= div;
        div /= 10;
    }

    result
}

fn main() {
    let case_1 = 12345;

    println!("case_1: {:?}", get_digit_back(case_1));
    println!("case_1: {:?}", get_digit_front(case_1));
}
