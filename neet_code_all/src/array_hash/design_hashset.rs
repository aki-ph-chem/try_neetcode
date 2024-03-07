// AC
struct MyHashSet {
    set_array: Vec<bool>,
}
impl MyHashSet {
    fn new() -> Self {
        let max_size = 10_usize.pow(6) + 1;
        Self {
            set_array: vec![false; max_size],
        }
    }

    fn add(&mut self, key: i32) {
        self.set_array[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.set_array[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.set_array[key as usize]
    }
}

fn main() {
    let mut my_hash_set_1 = MyHashSet::new();
    my_hash_set_1.add(1);
    my_hash_set_1.add(2);
    println!("my_hash_set_1.contains(1): {}", my_hash_set_1.contains(1));
    println!("my_hash_set_1.contains(3): {}", my_hash_set_1.contains(3));
    my_hash_set_1.remove(2);
    println!("my_hash_set_1.contains(2): {}", my_hash_set_1.contains(2));
}
