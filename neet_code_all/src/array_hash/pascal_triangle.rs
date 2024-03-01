struct Solution {}
impl Solution {
    // AC
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];
        if num_rows == 1 {
            return result;
        }

        for n in 1..num_rows {
            let mut new_row = vec![0; n as usize + 1];
            new_row[0] = 1;
            new_row[n as usize] = 1;

            for i in 1..n {
                new_row[i as usize] =
                    result[n as usize - 1][i as usize - 1] + result[n as usize - 1][i as usize];
            }
            result.push(new_row);
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for line in 1..=num_rows {
            let mut c = 1;
            let mut v = vec![];

            for i in 1..=line {
                v.push(c);
                c = c * (line - i) / i;
            }
            result.push(v);
        }

        result
    }
}

//C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        for i in 0..num_rows {
            let mut row = vec![1; i as usize + 1];
            for j in 1..i {
                row[j as usize] =
                    result[i as usize - 1][j as usize] + result[i as usize - 1][j as usize - 1];
            }

            result.push(row);
        }

        result
    }
}

fn main() {
    println!("case_1: {:?}", Solution::generate(2));
    println!("case_1: {:?}", Solution::generate(3));
    println!("case_1: {:?}", Solution::generate(4));
    println!("case_1: {:?}", Solution::generate(5));

    println!("case_1: {:?}", SolutionAns::generate(2));
    println!("case_1: {:?}", SolutionAns::generate(3));
    println!("case_1: {:?}", SolutionAns::generate(4));
    println!("case_1: {:?}", SolutionAns::generate(5));

    println!("case_1: {:?}", SolutionAnsCpp::generate(2));
    println!("case_1: {:?}", SolutionAnsCpp::generate(3));
    println!("case_1: {:?}", SolutionAnsCpp::generate(4));
    println!("case_1: {:?}", SolutionAnsCpp::generate(5));
}
