use std::collections::HashMap;

fn main() {

    let s_1 = "anagram".to_string();
    let t_1 = "nagaram".to_string();
    let mut map: HashMap<char, i64> = HashMap::new();

    for (a, b) in s_1.chars().zip(t_1.chars()) {
        *map.entry(a).or_default()+= 1;
        *map.entry(b).or_default()+= 1;
    }

    let flag = map.into_values().all(|cnt| cnt == 0);
    println!("flag: {}", flag);
}
