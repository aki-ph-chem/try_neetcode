use std::collections::HashMap;

struct Solution {}
impl Solution {
    // AC
    pub fn find_rpeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }

        let n = s.len();
        let mut map: HashMap<String, i32> = HashMap::new();

        for i in 0..=n - 10 {
            //println!("i,sub_str: {i}, {}", s[i..(i + 10)].to_string());
            *map.entry(s[i..(i + 10)].to_string().clone()).or_default() += 1;
        }
        //println!("map: {:?}", map);

        let mut result = vec![];
        for (sub_seq, n) in map {
            if n > 1 {
                result.push(sub_seq);
            }
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn find_rpeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() <= 10 {
            return vec![];
        }

        let n = s.len();
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut result = vec![];

        for i in 0..=(n - 10) {
            let sub_str = s[i..(i + 10)].to_string();
            *map.entry(sub_str.clone()).or_default() += 1;
            if let Some(n_sub_str) = map.get(&sub_str) {
                if *n_sub_str == 2 {
                    result.push(sub_str);
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
    // => ["AAAAACCCCC", "CCCCCAAAAA"]
    let case_2 = "AAAAAAAAAAAAA".to_string();
    // => ["AAAAAAAAAAAAA"]
    let case_3 = "AAAAAAAAAAA".to_string();
    // => ["AAAAAAAAAA"]

    println!(
        "case_1: {:?}",
        Solution::find_rpeated_dna_sequences(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_rpeated_dna_sequences(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        Solution::find_rpeated_dna_sequences(case_3.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::find_rpeated_dna_sequences(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::find_rpeated_dna_sequences(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::find_rpeated_dna_sequences(case_3.clone())
    );
}
