use std::collections::{HashSet,HashMap};

// 解けなかった😭
/*
struct Solution {}
impl Solution {
    pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
        for v in &strs {
        }
        vec![vec![]]
    }
}
*/

struct SolutionAns {}
impl SolutionAns {
    pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs{
            let mut key = [0_u8; 26];

            for c in s.chars(){
                key[c as usize - 'a' as usize] += 1;
            }

            if let Some(vals) = map.get_mut(&key){
                vals.push(s);
            }else{
                map.insert(key, vec![s]);
            }
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 適切なテストがわからない
    #[test]
    fn test_ans_1() {
        let input = vec!["eat","tea","tan","ate","nat","bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let output = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(),"tan".to_string()],
            vec!["ate".to_string(),"eat".to_string(),"tea".to_string()]
        ];
        assert_eq!(SolutionAns::group_anagram(input), output);
    }
}

fn main() {
}
