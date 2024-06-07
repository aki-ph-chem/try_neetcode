#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include <unordered_map>

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

class SolutionAns {
    public:
        bool isValid(std::string s) {
            std::stack<char> open;
            std::unordered_map<char, char> parens = {
                {')', '('},
                {']', '['},
                {'}', '{'},
            };

            for (const auto& c : s) {
                if (parens.find(c) != parens.end()) {
                    // if input starts with a closing bracket.
                    if (open.empty()) {
                        return false;
                    }

                    if (open.top() != parens[c]) {
                        return false;
                    }

                    open.pop();
                } else {
                    open.push(c);
                }
            }

            return open.empty();
        }
};


// 後で時間を置いて解いたときの解
class SolutionLatter {
    public:
        // AC
        bool isValid(std::string s) {
            std::stack<char> stack;
            for(auto& c: s) {
                if(!stack.empty()) {
                    auto stack_top = stack.top();
                    if(stack_top == '(' && c == ')') {
                        stack.pop();
                    } else if(stack_top == '{' && c == '}') {
                        stack.pop();
                    } else if(stack_top == '[' && c == ']') {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                } else {
                    stack.push(c);
                }
            }

            return stack.empty();
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

    SolutionAns s_ans;
    std::cout <<"case_1: {}" << s_ans.isValid(case_1) << std::endl;
    std::cout <<"case_2: {}" << s_ans.isValid(case_2) << std::endl;
    std::cout <<"case_3: {}" << s_ans.isValid(case_3) << std::endl;
    std::cout <<"case_4: {}" << s_ans.isValid(case_4) << std::endl;

    SolutionLatter s_latter;
    std::cout <<"case_1: {}" << s_latter.isValid(case_1) << std::endl;
    std::cout <<"case_2: {}" << s_latter.isValid(case_2) << std::endl;
    std::cout <<"case_3: {}" << s_latter.isValid(case_3) << std::endl;
    std::cout <<"case_4: {}" << s_latter.isValid(case_4) << std::endl;
}
