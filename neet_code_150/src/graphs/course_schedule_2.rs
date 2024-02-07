use std::collections::{HashMap, HashSet};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        // グラフの構築
        for pair in prerequisites {
            if !graph.contains_key(&pair[1]) {
                graph.insert(pair[1], vec![pair[0]]);
            } else {
                if let Some(node) = graph.get_mut(&pair[1]) {
                    node.push(pair[0]);
                }
            }
        }

        let mut result = vec![];
        let mut visited: HashSet<i32> = HashSet::new();

        for course in 0..num_courses {
            Self::bfs(course, &mut visited, &mut result);
        }

        result
    }

    fn bfs(course: i32, visited: &mut HashSet<i32>, result: &mut Vec<i32>) {
        if visited.contains(&course) {
            return;
        }

        visited.insert(course);
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        // グラフの構築
        for pair in prerequisites {
            if !graph.contains_key(&pair[0]) {
                graph.insert(pair[0], vec![pair[1]]);
            } else {
                if let Some(nodes) = graph.get_mut(&pair[0]) {
                    nodes.push(pair[1]);
                }
            }
        }

        let mut visit: HashSet<i32> = HashSet::new();
        let mut cycle: HashSet<i32> = HashSet::new();

        let mut result = vec![];
        for c in 0..num_courses {
            if !Self::dfs(c, &graph, &mut visit, &mut cycle, &mut result) {
                return vec![];
            }
        }

        result
    }

    fn dfs(
        course: i32,
        graph: &HashMap<i32, Vec<i32>>,
        visit: &mut HashSet<i32>,
        cycle: &mut HashSet<i32>,
        result: &mut Vec<i32>,
    ) -> bool {
        if cycle.contains(&course) {
            return false;
        }

        if visit.contains(&course) {
            return true;
        }

        cycle.insert(course);

        // (1),(2) 共にAC

        // (1) 添字でアクセスする方法
        /*
        if let Some(courses) = graph.get(&course) {
            for i in 0..courses.len() {
                let next_course = courses[i];
                if !Self::dfs(next_course, graph, visit, cycle, result) {
                    return false;
                }
            }
        }
        */

        // (2) イテレータを使う方法
        if let Some(courses) = graph.get(&course) {
            for next_course in courses {
                if !Self::dfs(*next_course, graph, visit, cycle, result) {
                    return false;
                }
            }
        }

        cycle.take(&course);
        visit.insert(course);
        result.push(course);

        true
    }
}

fn main() {
    let case_1 = (2, vec![vec![1, 0]]);
    // => [0,1]
    let case_2 = (4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
    // => [0,2,1,3]

    println!(
        "case_1 {:?}",
        SolutionAnsCpp::find_order(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2 {:?}",
        SolutionAnsCpp::find_order(case_2.0, case_2.1.clone())
    );
}
