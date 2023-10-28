use std::collections::HashMap;

#[derive(Debug)]
struct TimeMap {
    value_set: HashMap<(String, i32), String>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { value_set: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.value_set.insert((value, timestamp), key);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let (mut left, mut right) = (0, timestamp);

        while left <= right {
            let mid = (left + right) / 2;

            if let Some(value) = self.value_set.get(&(key.clone(), mid)) {
                return value.to_string()
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

fn main() {
    let mut m_1 = TimeMap::new();

    m_1.set("foo".to_string(), "bar".to_string(), 1);
    m_1.get("foo".to_string(), 1);
    m_1.get("foo".to_string(), 3);
    m_1.set("foo".to_string(), "bar2".to_string(), 4);
    m_1.get("foo".to_string(), 4);
    m_1.get("foo".to_string(), 5);
}
