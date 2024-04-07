fn main() {
    let sum = (1..=10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    let sum = (1..=10).clone().fold(1000, |acc, x| acc + x);
    println!("sum = {}", sum);

    let array = vec![1, 3, 5, 0, 10, 6, 2, 3];
    let max = array.iter().fold(i32::MIN, |res, x| res.max(*x));
    let min = array.iter().fold(i32::MAX, |res, x| res.min(*x));

    println!("max: {}", max);
    println!("min: {}", min);

    let array = vec![1.2, 3.01, 5.3, 0.02, 10.5, 6.2, 2.1, 3.5];
    let max = array.iter().fold(f32::MIN, |res, x| res.max(*x));
    let min = array.iter().fold(f32::MAX, |res, x| res.min(*x));

    println!("max: {}", max);
    println!("min: {}", min);
}
