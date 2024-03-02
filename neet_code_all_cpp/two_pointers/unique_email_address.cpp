#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <set>
#include <regex>

class Solution {
    public:
        // AC
        int numUniqueEmails(std::vector<std::string>& emails) {
            std::unordered_set<std::string> set;

            for(auto& mail: emails) {
                int l = 0;
                std::string tmp = "";

                // local name 
                while(l < mail.size() && mail[l] != '@') {
                    if(mail[l] != '.' && mail[l] != '+') {
                        tmp.push_back(mail[l]);
                        ++l;
                    } else if(mail[l] == '.') {
                        ++l;
                    } else if (mail[l] == '+' || mail[l] == '@') {
                        break;
                    } 
                }

                // domain name
                while(l < mail.size() && mail[l] != '@') {
                    ++l;
                }
                while(l < mail.size()) {
                    tmp.push_back(mail[l]);
                    ++l;
                }

                set.insert(tmp);
            }

            return set.size();
        }
};

// 模範解答
// 模範解答の方が4倍ほど遅かった
class SolutionAns {
    public:
        int numUniqueEmails(std::vector<std::string>& emails) {
            std::set<std::string> unique_emails;
            for(auto& emial: emails) {
                // local name
                
                // それぞれ '@','+'から先を捨てる
                auto local_name = emial.substr(0, emial.find('@'));
                local_name = local_name.substr(0, emial.find('+'));

                // 正規表現を使って '.' => '' へと置換 
                local_name = std::regex_replace(local_name, std::regex("\\.") , ""); 

                // domain name
                auto domain_name = emial.substr(emial.find('@') + 1, emial.length());

                unique_emails.insert(local_name + '@' + domain_name);
            }

            return unique_emails.size();
        }
};

int main(void) {
    std::vector<std::string> case_1 = {
        "test.email+alex@leetcode.com",
        "test.e.mail+bob.cathy@leetcode.com",
        "testemail+david@lee.tcode.com",
    };
    // => 2 ("testemail@leetcode.com","testemail@lee.tcode.com")
    std::vector<std::string> case_2 = {
        "a@leetcode.com",
        "b@leetcode.com",
        "c@leetcode.com",
    };
    // => 3
    std::vector<std::string> case_3 = {
        "test.email+alex@leetcode.com",
        "test.email@leetcode.com",
    };
    // => 1
    
    Solution s_1;
    std::cout << s_1.numUniqueEmails(case_1) << std::endl;
    std::cout << s_1.numUniqueEmails(case_2) << std::endl;
    std::cout << s_1.numUniqueEmails(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.numUniqueEmails(case_1) << std::endl;
    std::cout << s_ans.numUniqueEmails(case_2) << std::endl;
    std::cout << s_ans.numUniqueEmails(case_3) << std::endl;
}
