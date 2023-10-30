#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
    bool is_valid(std::string s) {
        std::vector<char> state;

        for(auto& c: s) {
            switch (c) {
                case '(':
                    state.push_back(c);
                    break;

                case '{':
                    state.push_back(c);
                    break;

                case '[':
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

                case '}':
                    if(state.empty()) {
                        return false;
                    } else if(state.back() != '{'){
                        return false;
                    }
                    state.pop_back();
                    break;

                case ']':
                    if(state.empty()) {
                        return false;
                    } else if(state.back() != '['){
                        return false;
                    }
                    state.pop_back();
                    break;

                default:
                    return false;
                    break;
            }
        }

        if(!state.empty()) {
            return false;
        }

        return true;
    }
};

int main(void) {
    auto case_1 = "()";
    auto case_2 = "()[]{}";
    auto case_3 = "(]";
    auto case_4 = "([)]";

    Solution s_1;
    std::cout <<"case_1: {}" << s_1.is_valid(case_1) << std::endl;
    std::cout <<"case_2: {}" << s_1.is_valid(case_2) << std::endl;
    std::cout <<"case_3: {}" << s_1.is_valid(case_3) << std::endl;
    std::cout <<"case_4: {}" << s_1.is_valid(case_4) << std::endl;
}
