use std::collections::HashMap;

fn main() {
    // or_insert()
    // entry(key)のkeyが存在しないときにor_insert(v)の値が
    // (key, value)として挿入される
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("foo", 3);
    map.entry("piyo").or_insert(5);
    println!("map: {:?}", map);
    // "foo"の値に対して処理を行うことも可
    *map.entry("foo").or_insert(10) *= 100;
    println!("map: {:?}", map);

    // or_insert_with()
    // クロージャFを代入できる(F: () -> V)
    map.entry("poyo").or_insert_with(|| {
        let mut x = 10;
        x *= 111;
        x
    });
    println!("map: {:?}", map);
    *map.entry("poyo").or_insert_with(|| {
        let mut x = 10;
        x *= 1212;
        x
    }) /= 111;
    println!("map: {:?}", map);

    // or_insert_with_key()
    // クロージャFを代入できる(F: (&key) -> V)
    map.entry("xxooxx")
        .or_insert_with_key(|key| key.chars().count() as i32);
    println!("map: {:?}", map);

    // and_modify()
    // (key, value)のvalueを変更するクロージャーを適用できる
    map.entry("foo").and_modify(|e| *e /= 10).or_insert(1);
    println!("map: {:?}", map);
}
