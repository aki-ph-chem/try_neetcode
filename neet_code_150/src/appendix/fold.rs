fn main() {
    let sum = (1..=10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    let sum = (1..=10).clone().fold(1000, |acc, x| acc + x);
    println!("sum = {}", sum);
}
