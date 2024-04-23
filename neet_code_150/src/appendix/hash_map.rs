use std::collections::HashMap;

fn main() {
    let s_1 = "anagram".to_string();
    let mut map = HashMap::new();

    for c in s_1.chars() {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    for v in map.iter() {
        println!("v : {:?}", v);
    }

    let s_2 = "nagaram".to_string();

    // map.entry(x).or_dfault() += 1
    // xが含まれていたらなにもせずにvalueを1増やす
    // 一方、含まれていないならば、(x,0)を挿入してvalueを1増やす(1にする)
    let mut map_2: HashMap<char, i64> = HashMap::new();
    for (a, b) in s_1.chars().zip(s_2.chars()) {
        *map_2.entry(a).or_default() += 1;
        *map_2.entry(b).or_default() -= 1;
        println!("map_2: {:?}", map_2);
    }

    let map_3 = HashMap::from([("hoge", 3), ("fuga", 12), ("piyo", 13)]);
    // valuesのみの配列を得る
    let values = map_3.clone().into_values().collect::<Vec<i32>>();
    // keysのみの配列を得る
    let keys = map_3.into_keys().collect::<Vec<&str>>();

    println!("values: {:?}", values);
    println!("values: {:?}", values);

    let mut map_4 = HashMap::from([('a', 3), ('b', 4)]);
    // insert()メソッドの返り値はOption<T>
    // すでに存在するkeyによる(key,value)をinsertすると
    // insertする前のvalueをSome()でラップした値を返す
    println!("map_4.insert('a', 13): {:?}", map_4.insert('a', 13));
    // 存在しないkeyによる(key,value)をinsertするとNoneを返す
    println!("map_4.insert('x', 121): {:?}", map_4.insert('x', 121));
}
