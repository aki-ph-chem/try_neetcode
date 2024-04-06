use std::collections::HashMap;

struct Solution {}
impl Solution {
    // AC
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let mut map: HashMap<char, i32> = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        for c in p.chars() {
            *map.entry(c).or_default() += 1;
        }
        //println!("map: {:?}", map);

        let mut idx = 0;
        let mut map_tmp: HashMap<char, i32> = HashMap::new();
        let mut result = vec![];
        while idx < p.len() {
            *map_tmp.entry(s[idx]).or_default() += 1;
            idx += 1;
        }
        if map_tmp == map {
            result.push(0);
        }

        let mut left = 0;
        while idx < s.len() {
            *map_tmp.entry(s[idx]).or_default() += 1;
            if let Some(map_tmp_key) = map_tmp.get_mut(&s[left]) {
                if *map_tmp_key == 1 {
                    map_tmp.remove(&s[left]);
                } else {
                    *map_tmp_key -= 1;
                }
            }
            //println!("map_tmp: {:?}", map_tmp);

            if map_tmp == map {
                result.push(left as i32 + 1);
            }

            idx += 1;
            left += 1;
        }

        result
    }

    // 上のコードを少し整理した
    // AC
    pub fn find_anagrams_2(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut map_tmp: HashMap<char, i32> = HashMap::new();
        let mut result = vec![];

        for i in 0..p.len() {
            *map.entry(p[i]).or_default() += 1;
            *map_tmp.entry(s[i]).or_default() += 1;
        }
        if map_tmp == map {
            result.push(0);
        }

        let mut left = 0;
        for i in p.len()..s.len() {
            *map_tmp.entry(s[i]).or_default() += 1;
            if let Some(map_tmp_key) = map_tmp.get_mut(&s[left]) {
                if *map_tmp_key == 1 {
                    map_tmp.remove(&s[left]);
                } else {
                    *map_tmp_key -= 1;
                }
            }

            if map_tmp == map {
                result.push(left as i32 + 1);
            }

            left += 1;
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut map_p: HashMap<char, i32> = HashMap::new();
        let mut map_s: HashMap<char, i32> = HashMap::new();

        for i in 0..p.len() {
            *map_p.entry(p[i]).or_default() += 1;
            *map_s.entry(s[i]).or_default() += 1;
        }

        let mut result = vec![];
        let mut p_s = p.len();
        for i in 0..(s.len() - p.len() + 1) {
            if map_s == map_p {
                result.push(i as i32);
            }

            if p_s < s.len() {
                *map_s.entry(s[p_s]).or_default() += 1;
                p_s += 1;
            }

            if let Some(map_s_key) = map_s.get_mut(&s[i]) {
                *map_s_key -= 1;
            }
            if let Some(map_s_key) = map_s.get(&s[i]) {
                if *map_s_key == 0 {
                    map_s.remove(&s[i]);
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = ("cbaebabacd".to_string(), "abc".to_string());
    // => [0, 6]
    let case_2 = ("abab".to_string(), "ab".to_string());
    // => [0, 1, 2]

    println!(
        "case_1: {:?}",
        Solution::find_anagrams(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_anagrams(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {:?}",
        Solution::find_anagrams_2(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_anagrams_2(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::find_anagrams(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::find_anagrams(case_2.0.clone(), case_2.1.clone())
    );
}
