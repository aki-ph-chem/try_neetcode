use std::collections::HashMap;

struct Solution {}
impl Solution {
    // AC
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s: HashMap<char, Vec<i32>> = HashMap::new();
        let mut map_t: HashMap<char, Vec<i32>> = HashMap::new();

        for ((i, c_s), (_i, c_t)) in s.chars().enumerate().zip(t.chars().enumerate()) {
            if let Some(map_s_key) = map_s.get_mut(&c_s) {
                map_s_key.push(i as i32);
            } else {
                map_s.insert(c_s, vec![i as i32]);
            }

            if let Some(map_t_key) = map_t.get_mut(&c_t) {
                map_t_key.push(i as i32);
            } else {
                map_t.insert(c_t, vec![i as i32]);
            }
        }

        //println!("map_s: {:?}", map_s);
        //println!("map_t: {:?}", map_t);

        for (c_s, c_t) in s.chars().zip(t.chars()) {
            if let (Some(map_s_key), Some(map_t_key)) = (map_s.get(&c_s), map_t.get(&c_t)) {
                if map_s_key != map_t_key {
                    return false;
                }
            }
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut map_s_to_t, mut map_t_to_s) = (HashMap::new(), HashMap::new());

        for (c1, c2) in s.chars().zip(t.chars()) {
            if let Some(w) = map_s_to_t.insert(c1, c2) {
                if w != c2 {
                    return false;
                }
            }

            if let Some(w) = map_t_to_s.insert(c2, c1) {
                if w != c1 {
                    return false;
                }
            }
        }

        true
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m_1: HashMap<char, Vec<i32>> = HashMap::new();
        let mut m_2: HashMap<char, Vec<i32>> = HashMap::new();

        for (c_s, (i, c_t)) in s.chars().zip(t.chars().enumerate()) {
            if let Some(map_s_key) = m_1.get_mut(&c_s) {
                map_s_key.push(i as i32);
            } else {
                m_1.insert(c_s, vec![i as i32]);
            }
            if let Some(map_t_key) = m_2.get_mut(&c_t) {
                map_t_key.push(i as i32);
            } else {
                m_2.insert(c_t, vec![i as i32]);
            }

            if let (Some(m_1_key), Some(m_2_key)) = (m_1.get(&c_s), m_2.get(&c_t)) {
                if m_1_key != m_2_key {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let case_1 = ("egg".to_string(), "add".to_string());
    // => true
    let case_2 = ("foo".to_string(), "bar".to_string());
    // => false
    let case_3 = ("paper".to_string(), "title".to_string());
    // => true

    println!(
        "case_1: {}",
        Solution::is_isomorphic(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::is_isomorphic(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::is_isomorphic(case_3.0.clone(), case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::is_isomorphic(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::is_isomorphic(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::is_isomorphic(case_3.0.clone(), case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::is_isomorphic(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::is_isomorphic(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::is_isomorphic(case_3.0.clone(), case_3.1.clone())
    );
}
