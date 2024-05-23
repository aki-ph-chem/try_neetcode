use std::collections::HashMap;

// 解けなかった
struct Solution;
impl Solution {
    // O(N^3)
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res_list = vec![];
        for p_1 in &points {
            for p_2 in &points {
                let mut num_point = 0;
                let (d_x_1, d_y_1) = (p_2[0] - p_1[0], p_2[1] - p_1[1]);
                for p in &points {
                    let (d_x_2, d_y_2) = (p[0] - p_1[0], p[1] - p_1[1]);
                    if d_x_1 == 0 || d_y_1 == 0 || d_x_2 == 0 || d_y_2 == 0 {
                        continue;
                    }
                }
            }
        }

        println!("res_list: {:?}", res_list);
        *res_list.iter().max().unwrap()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    // O(N^2)
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 1;

        for i in 0..points.len() {
            let p_1 = &points[i];
            let mut count = HashMap::new();
            for j in (i + 1)..points.len() {
                let p_2 = &points[j];

                let slope = if p_2[0] == p_1[0] {
                    i32::MAX
                } else {
                    (((p_2[1] as f64 - p_1[1] as f64) / (p_2[0] as f64 - p_1[0] as f64)) * 100000.0)
                        as i32
                };

                *count.entry(slope).or_insert(1) += 1;
                result = result.max(*count.get(&slope).unwrap());
            }
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 1;

        for i in 0..points.len() {
            let mut count: HashMap<i32, i32> = HashMap::new();
            for j in (i + 1)..points.len() {
                let s = Self::slope(&points[i], &points[j]);
                *count.entry(s).or_default() += 1;
                if let Some(num_s) = count.get(&s) {
                    result = result.max(num_s + 1);
                };
            }
        }

        result
    }

    fn slope(p_1: &Vec<i32>, p_2: &Vec<i32>) -> i32 {
        if p_2[0] == p_1[0] {
            return i32::MAX;
        }

        (((p_2[1] as f64 - p_1[1] as f64) / (p_2[0] as f64 - p_1[0] as f64)) * 100000.0) as i32
    }
}

fn main() {
    let case_1 = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    // => 3
    let case_2 = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    // => 4

    //println!("case_1: {}", Solution::max_points(case_1.clone()));
    //println!("case_2: {}", Solution::max_points(case_2.clone()));

    println!("case_1: {}", SolutionAns::max_points(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_points(case_2.clone()));

    println!("case_1: {}", SolutionAnsCpp::max_points(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::max_points(case_2.clone()));
}
