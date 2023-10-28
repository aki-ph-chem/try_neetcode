use std::collections::HashMap;

#[derive(Debug)]
struct TimeMap {
    value_set: HashMap<(String, i32), String>,
}

// ダメ
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            value_set: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.value_set.insert((value, timestamp), key);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let (mut left, mut right) = (0, timestamp);

        while left <= right {
            let mid = (left + right) / 2;

            if let Some(value) = self.value_set.get(&(key.clone(), mid)) {
                return value.to_string();
            }

            if mid < timestamp {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        eprintln!("");
        "".to_string()
    }
}

// TLE
#[derive(Debug)]
pub struct TimeMapB {
    value_set: HashMap<String, HashMap<i32, String>>,
}

impl TimeMapB {
    fn new() -> Self {
        TimeMapB {
            value_set: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.value_set.get_mut(&key) {
            Some(v) => {
                (*v).insert(timestamp, value);
            }
            None => {
                self.value_set
                    .insert(key, HashMap::from([(timestamp, value)]));
            }
        }
    }

    // TLE
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(time_and_value) = self.value_set.get(&key) {
            let mut res = "".to_string();
            for t in (1..timestamp + 1).rev() {
                match time_and_value.get(&t) {
                    Some(v) => {
                        res = v.to_string();
                        break;
                    }
                    None => {}
                }
            }
            eprintln!("{res}");
            res
        } else {
            eprintln!("");
            "".to_string()
        }
    }
}

// AC
#[derive(Debug)]
pub struct TimeMapC {
    value_set: HashMap<String, Vec<(i32, String)>>,
}
impl TimeMapC {
    fn new() -> Self {
        TimeMapC {
            value_set: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.value_set.get_mut(&key) {
            Some(v) => {
                (*v).push((timestamp, value));
            }
            None => {
                self.value_set.insert(key, vec![(timestamp, value)]);
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(time_and_value) = self.value_set.get(&key) {
            let (mut left, mut right) = (0, time_and_value.len() as i32 - 1);
            let mut mid = (left + right) / 2;

            while left <= right {
                if time_and_value[mid as usize].0 <= timestamp {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
                mid = (left + right) / 2;
            }

            let res = if time_and_value[mid as usize].0 <= timestamp {
                time_and_value[mid as usize].1.to_string()
            } else {
                "".to_string()
            };

            eprintln!("{res}");
            res
        } else {
            eprintln!("");
            "".to_string()
        }
    }
}

struct TimeMapAns {
    hm: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMapAns {
    fn new() -> Self {
        Self { hm: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.hm.entry(key).or_default().push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut res = String::new();

        if let Some(t_list) = self.hm.get(&key) {
            let (mut l, mut r) = (0, t_list.len());

            while l < r {
                let m = l + (r - l) / 2;
                if timestamp < t_list[m].1 {
                    r = m;
                } else {
                    res = t_list[m].0.clone();
                    l = m + 1;
                }
            }
        }

        res
    }
}

fn main() {
    let mut m_3 = TimeMapC::new();
    m_3.set("foo".to_string(), "bar".to_string(), 1);

    m_3.get("foo".to_string(), 1); // bar
    m_3.get("foo".to_string(), 3); // bar

    m_3.set("foo".to_string(), "bar2".to_string(), 4);

    m_3.get("foo".to_string(), 4); // bar2
    m_3.get("foo".to_string(), 5); // bar2
    println!("m_3: {:#?}", m_3);

    let mut m_3_2 = TimeMapC::new();

    m_3_2.set("love".to_string(), "high".to_string(), 10);
    m_3_2.set("love".to_string(), "low".to_string(), 20);

    m_3_2.get("love".to_string(), 5);
    m_3_2.get("love".to_string(), 10);
    m_3_2.get("love".to_string(), 15);
    m_3_2.get("love".to_string(), 20);
    m_3_2.get("love".to_string(), 25);

    let mut m_ans_1 = TimeMapAns::new();
    m_ans_1.set("foo".to_string(), "bar".to_string(), 1);

    println!("{}", m_ans_1.get("foo".to_string(), 1)); // bar
    println!("{}", m_ans_1.get("foo".to_string(), 3)); // bar

    m_ans_1.set("foo".to_string(), "bar2".to_string(), 4);

    println!("{}", m_ans_1.get("foo".to_string(), 4)); // bar2
    println!("{}", m_ans_1.get("foo".to_string(), 5)); // bar2
                                       
    let mut m_ans_2 = TimeMapAns::new();
    m_ans_2.set("love".to_string(), "high".to_string(), 10);
    m_ans_2.set("love".to_string(), "low".to_string(), 20);

    println!("{}",m_ans_2.get("love".to_string(), 5));
    println!("{}",m_ans_2.get("love".to_string(), 10));
    println!("{}",m_ans_2.get("love".to_string(), 15));
    println!("{}",m_ans_2.get("love".to_string(), 20));
    println!("{}",m_ans_2.get("love".to_string(), 25));
}
