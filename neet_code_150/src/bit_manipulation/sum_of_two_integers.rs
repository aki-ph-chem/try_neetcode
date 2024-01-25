// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        Self::rec(a,b)
    }

    fn rec(a: i32, b: i32) -> i32 {
        if (a & b) << 1 == 0 {
            return a ^ b;
        }

        Self::rec(a ^ b, (a & b) << 1)
    }
}

// C++の模範解答より
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carray = a & b;
            a ^= b;
            b = carray << 1;
        }

        a
    }
}

fn main() {
    let case_1 = (1, 2);
    //=> 3
    let case_2 = (2, 3);
    //=> 5
    println!("case_1: {}", SolutionAns::get_sum(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::get_sum(case_2.0, case_2.1));

    println!("case_1: {}", SolutionAnsCpp::get_sum(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAnsCpp::get_sum(case_2.0, case_2.1));
}
