// 解けなかった
struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let num_rows = num_rows as usize;
        let mut grid = vec![vec!['#'; n]; num_rows];

        let mut p = num_rows - 1;
        for r in 0..num_rows {
            let mut i = r;
            while i < n {
                grid[r][i] = s[i];
                i += 2 * p;
            }
            p -= 1;
            if p == 0 {
                p = num_rows - 1;
            }
        }

        let mut result = vec![];
        for r in 0..num_rows {
            for i in 0..n {
                if grid[r][i] != '#' {
                    result.push(grid[r][i]);
                }
            }
        }
        result.iter().collect()
    }
}

// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let s = s.chars().collect::<Vec<char>>();
        let mut result = "".to_string();
        for row in 0..num_rows as usize {
            let increment = 2 * (num_rows - 1);
            let mut i = row;
            while i < s.len() {
                result.push(s[i]);
                // ??
                if row > 0 && row < num_rows - 1 && i + increment - 2 * row < s.len() {
                    result.push(s[i + increment - 2 * row]);
                }

                i += increment;
            }
        }

        result
    }
}

fn main() {
    let case_1 = ("PAYPALISHIRING".to_string(), 3);
    // => "PAHNAPLSIIGYIR"
    let case_2 = ("PAYPALISHIRING".to_string(), 4);
    // => "PINALSIGYAHRPI"
    let case_3 = ("A".to_string(), 1);
    // => "A"

    /*
    println!("case_1: {}", Solution::convert(case_1.0.clone(), case_1.1));
    println!("case_2: {}", Solution::convert(case_2.0.clone(), case_2.1));
    */
    //println!("case_3: {}", Solution::convert(case_3.0.clone(), case_3.1));

    println!(
        "case_1: {}",
        SolutionAnsCpp::convert(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::convert(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::convert(case_3.0.clone(), case_3.1)
    );
}
