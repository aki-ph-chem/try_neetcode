// 解けなかった
struct Solutoin {}
impl Solutoin {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_array = nums
            .iter()
            .map(|x| Self::dgit_array(*x))
            .collect::<Vec<Vec<i32>>>();
        nums_array.sort_by(|x, y| Self::cmp_array(x, y));

        println!("nums_array: {:?}", nums_array);
        "foo".to_string()
    }

    fn dgit_array(mut num: i32) -> Vec<i32> {
        let mut result = vec![];
        while num > 0 {
            result.push(num % 10);
            num /= 10;
        }

        result.reverse();
        result
    }

    fn cmp_array(digits_1: &Vec<i32>, digits_2: &Vec<i32>) -> std::cmp::Ordering {
        let (array_1, array_2) = if digits_1.len() < digits_2.len() {
            (digits_1, digits_2)
        } else {
            (digits_2, digits_1)
        };

        let mut i = 0;
        while i < array_1.len() {
            if digits_1[i] > digits_2[i] {
                return std::cmp::Ordering::Greater;
            }
            if digits_1[i] < digits_2[i] {
                return std::cmp::Ordering::Less;
            }
            i += 1;
        }

        if let Some(array_2_last) = array_2.last() {
            if *array_2_last == 0 {
                return std::cmp::Ordering::Greater;
            }
        }

        if i < array_2.len() {
            return std::cmp::Ordering::Greater;
        }

        std::cmp::Ordering::Equal
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut v = nums.iter().map(|&n| n.to_string()).collect::<Vec<String>>();
        v.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
        //println!("v: {:?}", v);

        if v[0] == "0" {
            return "0".to_string();
        }

        v.join("")
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut v = nums.iter().map(|&n| n.to_string()).collect::<Vec<String>>();
        v.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));

        if v[0] == "0" {
            return "0".to_string();
        }

        let mut result = "".to_string();
        for s in &v {
            result += s;
        }

        result
    }
}

fn main() {
    let case_1 = vec![10, 2];
    // => "210"
    let case_2 = vec![3, 30, 34, 5, 9];
    // => "9534330"

    /*
    println!("case_1: {}", Solutoin::largest_number(case_1.clone()));
    println!("case_2: {}", Solutoin::largest_number(case_2.clone()));
    */

    println!("case_1: {}", SolutionAns::largest_number(case_1.clone()));
    println!("case_2: {}", SolutionAns::largest_number(case_2.clone()));

    println!("case_1: {}", SolutionAnsCpp::largest_number(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::largest_number(case_2.clone()));
}
