use std::collections::HashMap;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        1
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut end_count: HashMap<i32, i32> = HashMap::new();
        let (mut max_end_count, rows) = (0, wall.len());

        for i in 0..rows {
            let mut end_of_brick = 0;
            let cols = wall[i].len() - 1;

            for j in 0..cols {
                end_of_brick += wall[i][j];

                if let Some(end_count_key) = end_count.get_mut(&end_of_brick) {
                    *end_count_key += 1;
                } else {
                    end_count.insert(end_of_brick, 1);
                }

                max_end_count = max_end_count.max(end_count[&end_of_brick]);
            }
        }

        rows as i32 - max_end_count
    }
}

fn main() {
    let case_1 = vec![
        vec![1, 2, 2, 1],
        vec![3, 1, 2],
        vec![1, 3, 2],
        vec![2, 4],
        vec![3, 1, 2],
        vec![1, 3, 1, 1],
    ];
    // => 2
    let case_2 = vec![vec![1], vec![1], vec![1]];
    // => 3

    println!("case_1: {}", SolutionAnsCpp::least_bricks(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::least_bricks(case_2.clone()));
}
