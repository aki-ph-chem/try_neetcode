use std::collections::HashMap;

fn main() {
    // HashMap<U, V>のときのentry(x),or_default()
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("foo").or_default();
    *map.entry("hoge").or_default() += 1;
    println!("map: {:?}", map);

    // HashMap<U, Vec<V>>のときのentry(x),or_default()
    let mut map_2: HashMap<i32, Vec<i32>> = HashMap::new();
    map_2.insert(3, vec![1, 2]);
    map_2.entry(3).or_default().push(3);
    println!("map_2: {:?}", map_2);

    map_2.entry(4).or_default().push(1);
    println!("map_2: {:?}", map_2);

    map_2.entry(12).or_default();
    println!("map_2: {:?}", map_2);
}
