// Hint 1
// この問題は有向グラフにおいて有向サイクルが存在するかどうかの判定問題に等しい
// この場合はトポロジカルソートはできない
//
// Hint 2
// DFSによるトポロジカルソートを参考にせよ
//
// Hint 3
// BFSでもまたトポロジカルソートは実装可能である
use std::collections::{hash_map::HashMap, hash_set::HashSet};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn cna_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let last_course = num_courses - 1;
        true
    }
}

// C++の模範解答より
// runtime errorを修正中 => AC
struct SolutionAns {}
impl SolutionAns {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
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
        if visited.contains(&course) {
            return false;
        }

        if let None = map.get(&course) {
            return true;
        }

        visited.insert(course);

        for i in 0..map[&course].len() {
            let next_couse = map[&course][i];
            if !Self::dfs(next_couse, map, visited) {
                return false;
            }
        }

        if let Some(v) = map.get_mut(&course) {
            v.clear();
        }

        visited.take(&course);
        true
    }
}

// GraphをHashMap<i32,Vec<i32>>ではなくてVec<Vec<i3>>で表現
// グラフの構築が少し楽
struct SolutionAns2 {}
impl SolutionAns2 {
    // AC
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![]; num_courses as usize];

        for edge in &prerequisites {
            let (from, to) = (edge[0], edge[1]);
            graph[from as usize].push(to);
        }

        let mut visited = HashSet::new();
        for couse in 0..num_courses {
            if !Self::dfs(couse, &mut graph, &mut visited) {
                return false;
            }
        }

        true
    }

    fn dfs(course: i32, graph: &mut Vec<Vec<i32>>, visited: &mut HashSet<i32>) -> bool {
        if visited.contains(&course) {
            return false;
        }
        if graph[course as usize].is_empty() {
            return true;
        }

        visited.insert(course);
        for i in 0..graph[course as usize].len() {
            let next_course = graph[course as usize][i];
            if !Self::dfs(next_course, graph, visited) {
                return false;
            }
        }

        if !graph[course as usize].is_empty() {
            graph[course as usize].clear();
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
    let case_3 = (
        20,
        vec![
            vec![0, 10],
            vec![3, 18],
            vec![5, 5],
            vec![6, 11],
            vec![11, 14],
            vec![13, 1],
            vec![15, 1],
            vec![17, 4],
        ],
    );
    // => false

    println!(
        "case_1: {}",
        SolutionAns::can_finish(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::can_finish(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::can_finish(case_3.0, case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns2::can_finish(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns2::can_finish(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns2::can_finish(case_3.0, case_3.1.clone())
    );
}
