use std::collections::HashSet;

struct Solution {}
impl Solution {
    // AC
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let emails: Vec<Vec<char>> = emails.iter().map(|x| x.chars().collect()).collect();
        let mut set: HashSet<Vec<char>> = HashSet::new();

        for mail in emails {
            let mut l = 0;
            let mut tmp = vec![];
            // locla name
            while l < mail.len() && mail[l] != '@' {
                if mail[l] != '.' && mail[l] != '+' {
                    tmp.push(mail[l]);
                    l += 1;
                } else if mail[l] == '.' {
                    l += 1;
                } else if mail[l] == '+' || mail[l] == '@' {
                    break;
                }
            }

            // domain name
            while l < mail.len() && mail[l] != '@' {
                l += 1;
            }
            while l < mail.len() {
                tmp.push(mail[l]);
                l += 1;
            }
            //println!("{:?}", tmp);

            set.insert(tmp);
        }

        set.len() as i32
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once("@").unwrap();

            let mut local = local.split("+").take(1).next().unwrap().replace(".", "");
            local = format!("{}@{}", local, domain);

            unique_emails.insert(local);
        }

        unique_emails.len() as i32
    }
}

// e-mail アドレス
// @の前: local name, @の後: domain name
// local name:
// x.y, xy のように'.'があってもなくても同一視される
// '+' の後の文字列は無視される
// my+name@email.com とmy@email.comは同一視される

fn main() {
    let case_1 = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    // => 2 ("testemail@leetcode.com","testemail@lee.tcode.com")
    let case_2 = vec![
        "a@leetcode.com".to_string(),
        "b@leetcode.com".to_string(),
        "c@leetcode.com".to_string(),
    ];
    // => 3
    let case_3 = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.email@leetcode.com".to_string(),
    ];
    // => 1

    println!("case_1: {}", Solution::num_unique_emails(case_1.clone()));
    println!("case_2: {}", Solution::num_unique_emails(case_2.clone()));
    println!("case_3: {}", Solution::num_unique_emails(case_3.clone()));

    println!("case_1: {}", SolutionAns::num_unique_emails(case_1.clone()));
    println!("case_2: {}", SolutionAns::num_unique_emails(case_2.clone()));
    println!("case_3: {}", SolutionAns::num_unique_emails(case_3.clone()));
}
