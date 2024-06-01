#include <algorithm>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        std::string removeDuplicates(std::string s, int k) {
            std::stack<std::pair<char, int>> stack;

            for(auto& c: s) {
                int count = 1;
                if(!stack.empty() && stack.top().first == c) {
                    count += stack.top().second;
                    stack.pop();
                }

                stack.push({c, count});
                if(count == k) {
                    stack.pop();
                }
            }

            std::string result = "";
            while(!stack.empty()) {
                auto [c, freq] = stack.top();
                while(freq > 0) {
                    result += c;
                    --freq;
                }

                stack.pop();
            } 

            std::reverse(result.begin(), result.end());
            return result;
        }
};

// Rustの模範解答より
// AC
class SolutionAnsRust {
    public:
        std::string removeDuplicates(std::string s, int k) {
            std::vector<std::pair<char, int>> stack;
            for(auto&c : s) {
                if(!stack.empty() && stack.back().first == c) {
                    auto last = stack.back();
                    stack.pop_back();
                    ++last.second;
                    stack.push_back(last);
                } else {
                    stack.push_back({c, 1});
                }
                if(stack.back().second == k) {
                    stack.pop_back();
                }
            }

            std::string result = "";
            for(auto& [c, freq]: stack) {
                for(int i = 0; i < freq; ++i) {
                    result.push_back(c);
                }
            }
            return result;
        }
};

int main(void) {
    using Case = std::pair<std::string, int>;
    Case case_1 = {"abcd", 2};
    // => "abcd"
    Case case_2 = {"deeedbbcccbdaa", 3};
    // => "aa"
    Case case_3 = {"pbbcggttciiippooaais", 2};
    // => "ps"

    SolutionAns s_ans;
    std::cout << s_ans.removeDuplicates(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.removeDuplicates(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.removeDuplicates(case_3.first, case_3.second) << std::endl;

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.removeDuplicates(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_rs.removeDuplicates(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_rs.removeDuplicates(case_3.first, case_3.second) << std::endl;
}
