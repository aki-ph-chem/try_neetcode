// 解けなかった
struct Solution;
impl Solution {
    pub fn full_justify(word: Vec<String>, max_width: i32) -> Vec<String> {
        let mut len_strs = vec![];
        for s in &word {
            len_strs.push(s.len());
        }
        //println!("len_strs: {:?}", len_strs);
        let mut result = vec![];

        let mut len_tmp = 0;
        let mut s_tmp = "".to_string();
        for (s, s_len) in word.iter().zip(len_strs.iter()) {
            if len_tmp < max_width {
                len_tmp += *s_len as i32;
                s_tmp.push_str(&s);
            }
        }

        result
    }
}

// Pythonの模範解答より
struct SolutionAnsPython;
impl SolutionAnsPython {
    // AC
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = vec![];
        let mut line: Vec<String> = vec![];
        let mut length = 0;
        let mut i = 0;

        while i < words.len() {
            if length + line.len() + words[i].len() > max_width {
                let extra_space = max_width - length;
                let word_cnt = line.len() - 1;
                let spaces = extra_space / 1.max(word_cnt);
                let mut remainder = extra_space % 1.max(word_cnt);

                for j in 0..1.max(line.len() - 1) {
                    line[j] += &" ".repeat(spaces);
                    if remainder != 0 {
                        line[j] += " ";
                        remainder -= 1;
                    }
                }

                result.push(line.join(""));
                //println!("line: {:?}", line);
                line = vec![];
                length = 0;
            }

            line.push(words[i].clone());
            length += words[i].len();
            i += 1;
        }

        let last_line = line.join(" ");
        let tail_spaces = max_width - last_line.len();
        result.push(last_line + &" ".repeat(tail_spaces));

        result
    }
}

fn main() {
    let case_1 = (
        vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(),
        16,
    );
    // => ["This    is    an", "example  of text", "justification.  "]
    let case_2 = (
        vec!["What", "must", "be", "acknowledgment", "shall", "be"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        16,
    );
    // => ["What   must   be", "acknowledgment  ", "shall be        "]

    /*
    println!(
        "case_1: {:?}",
        Solution::full_justify(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::full_justify(case_2.0.clone(), case_2.1)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsPython::full_justify(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsPython::full_justify(case_2.0.clone(), case_2.1)
    );
}
