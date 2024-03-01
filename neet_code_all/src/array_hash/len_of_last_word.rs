// AC
struct Solution {}
impl Solution {
    pub fn lenght_of_last_word(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut r = 0;
        let mut w = 0;
        let mut w_list = vec![];

        while r < n {
            if s[r] == ' ' {
                r += 1;
            } else {
                while r < n && s[r] != ' ' {
                    r += 1;
                    w += 1;
                }

                w_list.push(w);
                w = 0;
            }
        }

        *w_list.last().unwrap()
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn lenght_of_last_word(s: String) -> i32 {
        // sの後ろの空白を取り除く
        let s = s.trim_end();
        let s: Vec<char> = s.chars().collect();
        let mut result = 0;

        // 後ろから空白でない間だけループを回す
        for i in (0..s.len()).rev() {
            if !s[i].is_whitespace() {
                result += 1;
            } else {
                break;
            }
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn length_of_last_word(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut ptr = n as i32 - 1;
        while ptr >= 0 && s[ptr as usize] == ' ' {
            ptr -= 1;
        }

        let mut len = 0;
        while ptr >= 0 && s[ptr as usize] != ' ' {
            len += 1;
            ptr -= 1;
        }

        len
    }
}

fn main() {
    let case_1 = "Hello World".to_string();
    // => 5
    let case_2 = "   fly me   to   the moon  ".to_string();
    // => 4
    let case_3 = "luffy is still joyboy".to_string();
    // => 6

    println!("case_1: {}", Solution::lenght_of_last_word(case_1.clone()));
    println!("case_2: {}", Solution::lenght_of_last_word(case_2.clone()));
    println!("case_3: {}", Solution::lenght_of_last_word(case_3.clone()));

    println!(
        "case_1: {}",
        SolutionAns::lenght_of_last_word(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::lenght_of_last_word(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::lenght_of_last_word(case_3.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::length_of_last_word(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::length_of_last_word(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::length_of_last_word(case_3.clone())
    );
}
