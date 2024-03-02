struct Solution {}
impl Solution {
    // AC
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        let mut result = String::new();
        let (mut w_1, mut w_2) = (0, 0);

        while w_1 < word1.len() && w_2 < word2.len() {
            result.push(word1[w_1]);
            result.push(word2[w_2]);

            w_1 += 1;
            w_2 += 1;
        }

        while w_2 < word2.len() {
            result.push(word2[w_2]);
            w_2 += 1;
        }

        while w_1 < word1.len() {
            result.push(word1[w_1]);
            w_1 += 1;
        }

        result
    }
}

fn main() {
    let case_1 = ("abc".to_string(), "pqr".to_string());
    // qpbqcr
    let case_2 = ("ab".to_string(), "pqrs".to_string());
    // apbqrs
    let case_3 = ("abcd".to_string(), "pq".to_string());
    // apbqcd

    println!(
        "case_1: {}",
        Solution::merge_alternately(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::merge_alternately(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::merge_alternately(case_3.0.clone(), case_3.1.clone())
    );
}
