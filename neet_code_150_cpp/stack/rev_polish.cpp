#include <iostream>
#include <vector>
#include <string>
#include <stack>

class Solution {
    public:
        int eval_rpn(std::vector<std::string>& toknes) {
            std::stack<int> stack;

            for(const auto &c: toknes) {
                if(c == "+") {
                    int a = stack.top();
                    stack.pop();
                    int b = stack.top();
                    stack.pop();
                    stack.push(b + a);
                }else if(c == "-") {
                    int a = stack.top();
                    stack.pop();
                    int b = stack.top();
                    stack.pop();
                    stack.push(b - a);
                }else if(c == "*") {
                    int a = stack.top();
                    stack.pop();
                    int b = stack.top();
                    stack.pop();
                    stack.push(b * a);
                }else if(c == "/") {
                    int a = stack.top();
                    stack.pop();
                    int b = stack.top();
                    stack.pop();
                    stack.push(b / a);
                } else {
                    stack.push(std::stoi(c));
                }
            }

            std::cout << stack.top() << std::endl;
            return stack.top();
        }
};

class SolutionAns {
public:
    int evalRPN(std::vector<std::string>& tokens) {
        std::stack<int> stk;
        
        for (int i = 0; i < static_cast<int>(tokens.size()); i++) {
            std::string token = tokens[i];
            
            if (token.size() > 1 || isdigit(token[0])) {
                stk.push(stoi(token));
                continue;
            }
            
            int num2 = stk.top();
            stk.pop();
            int num1 = stk.top();
            stk.pop();
            
            int result = 0;
            if (token == "+") {
                result = num1 + num2;
            } else if (token == "-") {
                result = num1 - num2;
            } else if (token == "*") {
                result = num1 * num2;
            } else {
                result = num1 / num2;
            }
            stk.push(result);
        }
        
        std::cout << stk.top() << std::endl;
        return stk.top();
    }
};

int main(void) {
    std::vector<std::string> case_1 = {"4", "13", "5", "/", "+"};
    std::vector<std::string> case_2 = {"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"};

    Solution s_1;
    s_1.eval_rpn(case_1);
    s_1.eval_rpn(case_2);

    SolutionAns s_ans;
    s_ans.evalRPN(case_1);
    s_ans.evalRPN(case_2);
}
