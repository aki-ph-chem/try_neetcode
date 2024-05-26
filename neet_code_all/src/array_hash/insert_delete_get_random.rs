use rand;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            return false;
        }

        self.set.insert(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.set.contains(&val) {
            return false;
        }

        self.set.take(&val);

        true
    }

    // Rustで乱数を使うにはrandクレートが要るのだが...
    pub fn get_random(&self) -> i32 {
        1
    }
}

// C++の模範解答より
// AC
struct RandomizedSetAnsCpp {
    indices: HashMap<i32, i32>,
    values: Vec<i32>,
}

impl RandomizedSetAnsCpp {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            values: vec![],
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if !self.indices.contains_key(&val) {
            self.values.push(val);
            self.indices.insert(val, self.values.len() as i32 - 1);
            return true;
        }

        false
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.indices.contains_key(&val) {
            return false;
        }

        let idx = self.indices[&val];
        if let Some(indices_vlaue) = self
            .indices
            .get_mut(&self.values[self.values.len() as usize - 1])
        {
            *indices_vlaue = idx;
        }
        self.values[idx as usize] = self.values[self.values.len() - 1];

        self.values.pop();
        self.indices.remove(&val);

        true
    }

    // LeetCodeでは流石にrandクレートは使えるようです..
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen::<usize>() % self.values.len();

        self.values[idx]
    }
}

fn main() {
    let mut r_ans_cpp = RandomizedSetAnsCpp::new();

    println!("{}", r_ans_cpp.insert(1));
    println!("{}", r_ans_cpp.remove(2));
    println!("{}", r_ans_cpp.insert(2));
    println!("{}", r_ans_cpp.get_random());
    println!("{}", r_ans_cpp.remove(1));
    println!("{}", r_ans_cpp.insert(2));
    println!("{}", r_ans_cpp.get_random());
}
