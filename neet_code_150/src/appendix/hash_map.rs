use std::collections::HashMap;

fn main() {
    let s_1 = "anagram".to_string();
    let mut map = HashMap::new();

    for c in s_1.chars() {
        map.entry(c)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    for v in map.iter() {
        println!("v : {:?}", v);
    }
}
