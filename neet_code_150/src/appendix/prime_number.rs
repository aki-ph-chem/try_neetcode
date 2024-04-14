// 素数判定
// time: O(sqrt(N))
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn main() {
    // 1 ~ 30まで
    for i in 1..=30 {
        println!("{i}, {}", is_prime(i));
    }
}
