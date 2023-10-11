use std::collections::HashMap;

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

// 模範解答
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

fn main() {
    let case_1 = vec!["eat","tea","tan","ate","nat","bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res_1 = SolutionAns::group_anagram(case_1);
    println!("res_1: {:?}", res_1);
    // expected: res_1
    /*
        vec!["bat".to_string()],
        vec!["nat".to_string(),"tan".to_string()],
        vec!["ate".to_string(),"eat".to_string(),"tea".to_string()]
    ];
    */

    let case_2 = vec!["".to_string()];
    let res_2 = SolutionAns::group_anagram(case_2);
    println!("res_2: {:?}", res_2);
    // expected: res_2
    /*
      vec![["".to_string()]]
    */

    let case_3 = vec!["a".to_string()];
    let res_3 = SolutionAns::group_anagram(case_3);
    println!("res_3: {:?}", res_3);
    // expected: res_3
    /*
     vec![["a".to_string()]]
     */
}
