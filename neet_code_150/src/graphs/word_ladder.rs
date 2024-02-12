use std::collections::{HashSet, VecDeque};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        2
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        for s in word_list {
            set.insert(s.into_bytes().clone());
        }

        let mut q: VecDeque<Vec<u8>> = VecDeque::new();
        q.push_back(begin_word.clone().into_bytes());

        let mut result = 1;

        while !q.is_empty() {
            let count = q.len();

            for _i in 0..count {
                let mut word = q.front().unwrap().clone();
                q.pop_front();

                if word == end_word.clone().into_bytes() {
                    return result;
                }
                set.take(&word);

                for j in 0..word.len() {
                    let c = word[j];

                    for k in 0..26 {
                        word[j] = k + b'a';

                        if set.contains(&word) {
                            q.push_back(word.clone());
                            set.take(&word);
                        }

                        word[j] = c;
                    }
                }
            }

            result += 1;
        }

        0
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_len = begin_word.len();
        let begin_word = Self::encode(begin_word);
        let end_word = Self::encode(end_word);

        let mut word_list: HashSet<u64> = word_list
            .into_iter()
            .map(|word| Self::encode(word))
            .collect();
        let mut frontier: VecDeque<u64> = VecDeque::with_capacity(5000);
        let mut seen: HashSet<u64> = HashSet::new();
        let mut res = 1;

        frontier.push_back(begin_word);
        while !frontier.is_empty() {
            let len = frontier.len();
            for _ in 0..len {
                let current = frontier.pop_front().unwrap();
                if current == end_word {
                    return res;
                }

                for i in 0..word_len {
                    let filter = !(0b11111 << (i * 5));
                    for character in 1..=26 {
                        let neighbour = ((current & filter) | (character << (i * 5)));
                        if word_list.contains(&neighbour) && !seen.contains(&neighbour) {
                            frontier.push_back(neighbour);
                            seen.insert(neighbour);
                        }
                    }
                }
            }

            res += 1;
        }

        0
    }

    fn encode(word: String) -> u64 {
        word.into_bytes().into_iter().fold(0, |mut acc, c| {
            acc <<= 5;
            acc | (c - b'a' + 1) as u64
        })
    }
}

fn main() {
    let case_1 = (
        "hit".to_string(),
        "cog".to_string(),
        vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ],
    );
    // => 5
    let case_2 = (
        "hit".to_string(),
        "cog".to_string(),
        vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ],
    );
    // => 0

    println!(
        "case_1 :{}",
        SolutionAnsCpp::ladder_length(case_1.0.clone(), case_1.1.clone(), case_1.2.clone())
    );
    println!(
        "case_2 :{}",
        SolutionAnsCpp::ladder_length(case_2.0.clone(), case_2.1.clone(), case_2.2.clone())
    );

    println!(
        "case_1 :{}",
        SolutionAns::ladder_length(case_1.0.clone(), case_1.1.clone(), case_1.2.clone())
    );
    println!(
        "case_2 :{}",
        SolutionAns::ladder_length(case_2.0.clone(), case_2.1.clone(), case_2.2.clone())
    );
}
