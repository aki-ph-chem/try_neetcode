#include <cwchar>
#include <iostream>
#include <stack>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        bool find132pattern(std::vector<int>& nums) {
            std::stack<std::pair<int, int>> stack;
            int current_min = nums[0];

            for(auto& n: nums) {
                while(!stack.empty() && stack.top().first <= n) {
                    stack.pop();
                }

                if(!stack.empty() && stack.top().second < n) {
                    return true;
                } else {
                    stack.push({n, current_min});
                    current_min = std::min(current_min, n);
                }
            }

            return false;
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        bool find132pattern(std::vector<int>& nums) {
            std::stack<std::pair<int, int>> stack;
            int current_min = nums[0];

            for(auto iter = ++(nums.begin()); iter != nums.end(); ++iter) {
                while(!stack.empty() && *iter >= stack.top().first) {
                        stack.pop();
                }

                if(!stack.empty() && *iter > stack.top().second) {
                    return true;
                }

                stack.push({*iter, current_min});
                current_min = std::min(current_min, *iter);
            }

            return false;
        }
};

int main(void) {
    std::vector<int> case_1 = {1, 2, 3, 4};
    // false
    std::vector<int> case_2 = {3, 1, 4, 2};
    // true
    std::vector<int> case_3 = {-1, 3, 2, 0};
    // true
    std::vector<int> case_4 = {1, 0, 1, -4, -3};
    // false
    std::vector<int> case_5 = {3, 5, 0, 3, 4};
    // true

    SolutionAns s_ans;
    std::cout << s_ans.find132pattern(case_1) << std::endl;
    std::cout << s_ans.find132pattern(case_2) << std::endl;
    std::cout << s_ans.find132pattern(case_3) << std::endl;
    std::cout << s_ans.find132pattern(case_4) << std::endl;
    std::cout << s_ans.find132pattern(case_5) << std::endl;

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.find132pattern(case_1) << std::endl;
    std::cout << s_ans_rs.find132pattern(case_2) << std::endl;
    std::cout << s_ans_rs.find132pattern(case_3) << std::endl;
    std::cout << s_ans_rs.find132pattern(case_4) << std::endl;
    std::cout << s_ans_rs.find132pattern(case_5) << std::endl;
}
