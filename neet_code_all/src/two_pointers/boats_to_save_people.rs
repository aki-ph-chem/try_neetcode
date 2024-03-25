// 解けなかった
struct Solution {}
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        let n = people.len() as i32 - 1;
        people.sort();

        let mut result = 0;
        let (mut left, mut right) = (n - 1, n);
        while 0 <= left && left <= right {
            if limit - people[right as usize] - people[left as usize] >= 0 {
                left -= 2;
                right -= 2;
                result += 1;
            } else {
                left -= 1;
                right -= 1;
                result += 1;
            }
        }

        result
    }
}

// AC
// C++の模範解答より: 気づけばオーソゾックスな尺取り法でした...
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();

        let mut result = 0;
        let (mut lightest_person, mut heaviest_person) = (0, people.len() as i32 - 1);

        while lightest_person <= heaviest_person {
            if people[lightest_person as usize] + people[heaviest_person as usize] <= limit {
                heaviest_person -= 1;
                lightest_person += 1;
            } else {
                heaviest_person -= 1;
            }
            result += 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 2], 3);
    // 1: ((1,2))
    let case_2 = (vec![3, 2, 2, 1], 3);
    // 3: ((1,2), (2), (3))
    let case_3 = (vec![3, 5, 3, 4], 5);
    // 4: ((3),(3),(4),(5))
    let case_4 = (vec![3, 2, 3, 2, 2], 6);
    // 3:

    println!(
        "case_1: {}",
        Solution::num_rescue_boats(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::num_rescue_boats(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::num_rescue_boats(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::num_rescue_boats(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::num_rescue_boats(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::num_rescue_boats(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::num_rescue_boats(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::num_rescue_boats(case_4.0.clone(), case_4.1)
    );
}
