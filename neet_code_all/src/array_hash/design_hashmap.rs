// AC
struct MyHashMap {
    map_array: Vec<(bool, i32)>,
}
impl MyHashMap {
    fn new() -> Self {
        let max_size = 10_usize.pow(6) + 1;
        Self {
            map_array: vec![(false, -1); max_size],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.map_array[key as usize].0 = true;
        self.map_array[key as usize].1 = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.map_array[key as usize].1
    }

    fn remove(&mut self, key: i32) {
        self.map_array[key as usize].0 = false;
        self.map_array[key as usize].1 = -1;
    }
}

fn main() {
    let mut my_hash_map_1 = MyHashMap::new();
    my_hash_map_1.put(1, 1);
    my_hash_map_1.put(2, 2);
    println!("my_hash_map_1.get(1): {}", my_hash_map_1.get(1));
    println!("my_hash_map_1.get(3): {}", my_hash_map_1.get(3));
    my_hash_map_1.put(2, 1);
    println!("my_hash_map_1.get(2): {}", my_hash_map_1.get(2));
    my_hash_map_1.remove(2);
    println!("my_hash_map_1.get(2): {}", my_hash_map_1.get(2));
}
