#include <iostream>
#include <string>
#include <vector>

// カッコの対応はstackで書ける
// ただ'*'があるときはどうしよう?
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
                        } else if(state.back() != '('){
                            return false;
                        }
                        state.pop_back();
                        break;

                    // '*'

                    defautl:
                        return false;
                }
            }

            if(!state.empty()) {
                return false;
            }

            return true;
        }
};

int main(void) {
    auto case_1 = std::string{"()()"};

    Solution s_1;
    std::cout << s_1.check_valid_string(case_1) << std::endl;
}
