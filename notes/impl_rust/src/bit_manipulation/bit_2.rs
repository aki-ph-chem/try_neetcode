fn is_odd(x: i32) -> bool {
    match x & 1 {
        0 => false,
        _ => true,
    }
}

fn main() {
    for n in 0..20 {
        let even_or_odd = if is_odd(n) { "odd" } else { "even" };
        println!("n = {n} is {even_or_odd}");
    }
}
