#include <iostream>
#include <string>

// Rustの模範解答より
class SolutionAnsRust {
    public:
        bool isSubsequence(std::string s, std::string t) {
            auto [t_l, s_l] = std::pair(0, 0);

            while(s_l < s.size() && t_l < t.size()) {
                if(s[s_l] == t[t_l]) {
                    ++s_l;
                    ++t_l;
                } else {
                    ++t_l;
                }
            }

            return s_l == s.size();
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isSubsequence(std::string s, std::string t) {
            auto [i, j] = std::pair(0,0);
            while(i < s.size() && j < t.size()) {
                if(s[i] == t[j]) {
                    ++i;
                }

                ++j;
            }

            if(i >= s.size()) {
                return true;
            } 
                
            return false;
        }
};

int main(void) {
    using Case = std::pair<std::string, std::string>;
    Case case_1 = {"abc", "ahbgdc"};
    // => true
    Case case_2 = {"axc", "ahbgdc"};
    // => false

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.isSubsequence(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_rs.isSubsequence(case_2.first, case_2.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.isSubsequence(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.isSubsequence(case_2.first, case_2.second) << std::endl;
}
