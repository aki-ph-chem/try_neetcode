// Hint 1
// この問題は有向グラフにおいて有向サイクルが存在するかどうかの判定問題に等しい 
// この場合はトポロジカルソートはできない
//
// Hint 2
// DFSによるトポロジカルソートを参考にせよ
//
// Hint 3
// BFSでもまたトポロジカルソートは実装可能である
use std::collections::{hash_set::HashSet, hash_map::HashMap};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn cna_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let last_course = num_courses - 1;
        true
    }
}

// C++の模範解答より
// runtime errorを修正中
struct SolutionAns {}
impl SolutionAns {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut map:HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in prerequisites {
            if !map.contains_key(&pair[0]) {
                map.insert(pair[0], vec![pair[1]]);
            } else {
                if let Some(v) = map.get_mut(&pair[0]) {
                    v.push(pair[1]);
                }
            }
        }

        let mut visited = HashSet::new();

        for couse in 0..num_courses {
            if !Self::dfs(couse, &mut map, &mut visited) {
                return false;
            }
        }

        true
    }

    fn dfs(course: i32, map: &mut HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> bool {
        if visited.contains(&course)  {
            return false;
        }

        /*
        if map[&course].is_empty() {
            return true;
        }
        */

        if let Some(map_couse) = map.get(&course) {
            if map_couse.is_empty() {
                return true;
            }
        }

        visited.insert(course);

        /*
        for i in 0..map[&course].len() {
            let next_couse = map[&course][i];
            if !Self::dfs(next_couse, map, visited) {
                return false;
            }
        }
        */

        if let Some(map_course) = map.get(&course) {
        }

        if let Some(v) =  map.get_mut(&course) {
            v.clear();
        }
        visited.take(&course);
        true
    }
}

fn main() {
    let case_1 = (2, vec![vec![1, 0]]);
    // => true
    let case_2 = (2, vec![vec![1, 0], vec![0, 1]]);
    // => false

    println!("case_1: {}", SolutionAns::can_finish(case_1.0, case_1.1.clone()));
    println!("case_2: {}", SolutionAns::can_finish(case_2.0, case_2.1.clone()));
}
