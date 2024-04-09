use std::collections::VecDeque;

// BFS,DFSの違いはtodoからpop_front()で取り出すか、pop_back()で取り出すかの違い

// BFS
fn search_bfs(graph: &Vec<Vec<i32>>, s: i32) {
    let n = graph.len();
    let mut seen = vec![false; n];
    let mut todo = VecDeque::new();

    seen[s as usize] = true;
    todo.push_back(s);

    while let Some(top) = todo.pop_front() {
        print!("{} ->", top);
        for x in &graph[top as usize] {
            if seen[*x as usize] {
                continue;
            }

            seen[*x as usize] = true;
            todo.push_back(*x);
        }
    }
    println!("");
}

// loopによるDFSの実装
fn search_dfs(graph: &Vec<Vec<i32>>, s: i32) {
    let n = graph.len();
    let mut seen = vec![false; n];
    let mut todo = VecDeque::new();

    seen[s as usize] = true;
    todo.push_back(s);

    while let Some(top) = todo.pop_back() {
        print!("{} ->", top);
        for x in &graph[top as usize] {
            if seen[*x as usize] {
                continue;
            }

            seen[*x as usize] = true;
            todo.push_back(*x);
        }
    }
    println!("");
}

fn main() {
    let graph = (5, vec![vec![1, 2], vec![3], vec![3], vec![4], vec![]]);

    println!("BFS");
    search_bfs(&graph.1, 0);

    println!("DFS");
    search_dfs(&graph.1, 0);
}
