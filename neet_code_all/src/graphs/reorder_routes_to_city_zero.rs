use std::collections::{HashSet, VecDeque};

// 解けなかった
struct Solution {}
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![HashSet::new(); n as usize];
        let mut graph_bi = vec![vec![]; n as usize];

        for edge in connections {
            let (from, to) = (edge[0], edge[1]);
            graph[from as usize].insert(to);

            graph_bi[from as usize].push(to);
            graph_bi[to as usize].push(from);
        }

        let mut visit = HashSet::new();
        let mut result = 0;
        Self::dfs(0, &graph, &graph_bi, &mut visit, &mut result);

        result
    }

    fn dfs(
        node: i32,
        graph: &Vec<HashSet<i32>>,
        graph_bi: &Vec<Vec<i32>>,
        visit: &mut HashSet<i32>,
        sum: &mut i32,
    ) {
        if visit.contains(&node) {
            return;
        }
        visit.insert(node);

        for next_node in &graph_bi[node as usize] {
            // 辺 next_node -> nodeが存在しないことはどう判定すればいい?
            if !graph[*next_node as usize].contains(&node) {
                *sum += 1;
                print!("from: {next_node} ");
                println!("to: {node}");
                println!("sum: {sum}");
            }

            Self::dfs(*next_node, graph, graph_bi, visit, sum);
        }
    }
}

//Javaの模範解答より(DFS)
struct SolutionAnsJava {}
impl SolutionAnsJava {
    // AC
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
        let mut old_edges: HashSet<(i32, i32)> = HashSet::new();
        let mut visit: HashSet<i32> = HashSet::new();

        for edge in connections {
            let (from, to) = (edge[0], edge[1]);
            old_edges.insert((from, to));
            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut result = 0;
        Self::dfs(0, -1, &graph, &old_edges, &mut visit, &mut result);

        result - 1
    }

    fn dfs(
        current_node: i32,
        parent_node: i32,
        graph: &Vec<Vec<i32>>,
        old_edges: &HashSet<(i32, i32)>,
        visited: &mut HashSet<i32>,
        result: &mut i32,
    ) {
        if visited.contains(&current_node) {
            return;
        }
        visited.insert(current_node);

        if !old_edges.contains(&(current_node, parent_node)) {
            *result += 1;
        }
        for child_node in &graph[current_node as usize] {
            Self::dfs(*child_node, current_node, graph, old_edges, visited, result);
        }
    }
}

// C++の模範解答より(BFS)
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        let mut visited = HashSet::new();

        for edge in connections {
            let (from, to) = (edge[0], edge[1]);
            graph[from as usize].push((to, true));
            graph[to as usize].push((from, false));
        }

        let mut result = 0;
        let mut stack = VecDeque::new();
        stack.push_back(0);
        visited.insert(0);

        while let Some(stack_top) = stack.pop_back() {
            for v in &graph[stack_top as usize] {
                if visited.contains(&v.0) {
                    continue;
                }

                stack.push_back(v.0);
                visited.insert(v.0);
                if v.1 {
                    result += 1;
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = (
        6,
        vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
    );
    // => 3
    let case_2 = (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]);
    // => 2
    let case_3 = (3, vec![vec![1, 0], vec![2, 0]]);
    // => 0

    /*
    println!(
        "case_1: {}",
        Solution::min_reorder(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::min_reorder(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::min_reorder(case_3.0, case_3.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsJava::min_reorder(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsJava::min_reorder(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsJava::min_reorder(case_3.0, case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::min_reorder(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::min_reorder(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::min_reorder(case_3.0, case_3.1.clone())
    );
}
