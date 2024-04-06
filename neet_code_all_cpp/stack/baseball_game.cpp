#include <cwchar>
#include <iostream>
#include <string>
#include <vector>
#include <stack>

class Solution {
    public:
        // AC
        // std::vectorを使った実装
        int calPoints(std::vector<std::string>& operations) {
            std::vector<int> stack;

            for(auto &op: operations) {
                if(op == "C") {
                    stack.pop_back();
                } else if (op == "D") {
                    auto d_top = 2 * stack.back();
                    stack.push_back(d_top);
                } else if (op == "+") {
                    auto n = stack.size();
                    int sum = stack[n - 1] + stack[n - 2];
                    stack.push_back(sum);
                } else {
                    int num = std::stoi(op);
                    stack.push_back(num);
                }
            }
            int result = 0;
            for(auto& v: stack) {
                result += v;
            }

            return result;
        }

        // AC
        // std::stackを使った実装
        int calPoints2(std::vector<std::string>& operations) {
            std::stack<int> stack;

            for(auto &op: operations) {
                if(op == "C") {
                    stack.pop();
                } else if (op == "D") {
                    auto d_top = 2 * stack.top();
                    stack.push(d_top);
                } else if (op == "+") {
                    auto top_prev = stack.top();
                    stack.pop();
                    int sum = top_prev + stack.top();
                    stack.push(top_prev);
                    stack.push(sum);
                } else {
                    int num = std::stoi(op);
                    stack.push(num);
                }
            }
            int result = 0;
            while(!stack.empty()) {
                result += stack.top();
                stack.pop();
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int calPoints(std::vector<std::string>& operations) {
            std::stack<int> stack;
            int result = 0;

            for(auto& op: operations) {
                if(op == "+") {
                    auto first = stack.top();
                    stack.pop();
                    auto second = stack.top();
                    stack.push(first);
                    stack.push(first + second);
                    result += first + second;
                } else if(op == "D") {
                    result += 2 * stack.top();
                    stack.push(2 * stack.top());
                } else if(op == "C") {
                    result -= stack.top();
                    stack.pop();
                } else {
                    result += std::stoi(op);
                    stack.push(std::stoi(op));
                }
            }

            return result;
        }
};

int main(void) {
    std::vector<std::string> case_1 = {"5","2","C","D","+"};
    // => 30
    std::vector<std::string> case_2 = {"5","-2","4","C","D","9","+","+"};
    // => 27
    std::vector<std::string> case_3 = {"1","C"};
    // => 0

    Solution s_1;

    std::cout << s_1.calPoints(case_1) << std::endl;
    std::cout << s_1.calPoints(case_2) << std::endl;
    std::cout << s_1.calPoints(case_3) << std::endl;

    std::cout << s_1.calPoints2(case_1) << std::endl;
    std::cout << s_1.calPoints2(case_2) << std::endl;
    std::cout << s_1.calPoints2(case_3) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.calPoints(case_1) << std::endl;
    std::cout << s_ans.calPoints(case_2) << std::endl;
    std::cout << s_ans.calPoints(case_3) << std::endl;
}
