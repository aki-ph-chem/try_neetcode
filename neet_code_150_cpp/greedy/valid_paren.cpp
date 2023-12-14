#include <iostream>
#include <string>
#include <vector>

// カッコの対応はstackで書ける
// ただ'*'があるときはどうしよう?
// '*'は文字''としての意味と、')' or ')'としての意味の両方を持つ
class Solution {
    public:
        bool check_valid_string(std::string s) {
            std::vector<char> state;

            for(auto& c: s) {
                switch(c) {
                    case '(':
                        state.push_back(c);
                        break;

                    case ')':
                        if(state.empty()) {
                            return false;
                        } else if(state.back() != '(' && state.back() != '*'){
                            return false;
                        }
                        state.pop_back();
                        break;

                    // '*'
                    case '*':
                        state.push_back(c);
                        break;
                }
            }

            if(!state.empty()) {
                return false;
            }

            return true;
        }
};

class SolutionAns {
    public:
        bool checkValidString(std::string s) {
            int n = s.size();

            // 文字列の頭から balancedを'(' or '*'ならインクリメント、それ以外ならデクリメント
            int balanced = 0;
            for(int i=0; i<n; i++) {
                if(s[i] == '(' || s[i] == '*') 
                    balanced++;
                else 
                    balanced--;

                if(balanced < 0) 
                    return false;
            }

            // バランスが取れていればok
            if(balanced == 0) 
                return true;

            // 文字列の後ろからbalanceを')'or'*'ならばインクリメント、それ以外ならデクリメント
            balanced = 0;
            for(int i=n-1; i>=0; i--) {
                if(s[i] == ')' || s[i] == '*') 
                    balanced++;
                else 
                    balanced--;

                if(balanced < 0) 
                    return false;
            }

            return true;
        }
};

int main(void) {
    auto case_1 = std::string{"()()"}; // true
    auto case_2 = std::string{"())"}; // true
    auto case_3 = std::string{"*)"}; // true
    auto case_4 = std::string{"(*"}; // true

    /*
    Solution s_1;
    std::cout << s_1.check_valid_string(case_1) << std::endl;
    std::cout << s_1.check_valid_string(case_2) << std::endl;
    std::cout << s_1.check_valid_string(case_3) << std::endl;
    std::cout << s_1.check_valid_string(case_4) << std::endl;
    */

    SolutionAns s_ans;
    std::cout << s_ans.checkValidString(case_1) << std::endl;
    std::cout << s_ans.checkValidString(case_2) << std::endl;
    std::cout << s_ans.checkValidString(case_3) << std::endl;
    std::cout << s_ans.checkValidString(case_4) << std::endl;
}
