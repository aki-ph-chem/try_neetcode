#include <algorithm>
#include <iostream>
#include <vector>
#include <stack>

// Rustの模範解答より
class SolutionAnsRust {
    public:
        std::vector<int> daily_tmp(const std::vector<int>& temperatures) {
            auto len_tmp = temperatures.size();
            std::vector<int> result(len_tmp,0);
            std::stack<std::pair<int,std::size_t>> stack;

            for(std::size_t i = 0; i < len_tmp; ++i) {
                while(!stack.empty() && temperatures[i] > stack.top().first) {
                    auto stack_idx = stack.top().second;
                    stack.pop();
                    result[stack_idx] = static_cast<int>(i - stack_idx);
                }

                stack.push({temperatures[i], i});
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> dailyTemperatures(const std::vector<int>& temperatures) {
            int n = temperatures.size();

            // pair: [index, temp]
            std::stack<std::pair<int, int>> stk;
            std::vector<int> result(n);

            for (int i = 0; i < n; i++) {
                int currDay = i;
                int currTemp = temperatures[i];

                while (!stk.empty() && stk.top().second < currTemp) {
                    int prevDay = stk.top().first;
                    int prevTemp = stk.top().second;
                    stk.pop();

                    result[prevDay] = currDay - prevDay;
                }

                stk.push({currDay, currTemp});
            }

            return result;
        }
};

// 構造化束縛を使うとちょっとスッキリ
class SolutionRe {
    public:
        std::vector<int> dailyTemperatures(const std::vector<int>& temperatures) {
            auto n = temperatures.size();
            
            std::stack<std::pair<int, int>> stk;
            std::vector<int> result(n);

            for(int i = 0; i < n; ++i) {
                int currDay = i;
                int currTemp = temperatures[i];

                while(!stk.empty() && stk.top().second < currTemp) {
                    // 構造化束縛!
                    auto [prevDay, prevTemp] = stk.top();
                    stk.pop();

                    result[prevDay] = currDay - prevDay;
                }

                stk.push({currDay, currTemp});
            }

            return result;
        }
};

int main(void) {
    const auto case_1 = std::vector{73, 74, 75, 71, 69, 72, 76, 73};
    const auto case_2 = std::vector{30, 40, 50, 60};
    const auto case_3 = std::vector{30, 60, 90};

    SolutionAnsRust s_ans_rust;

    auto res_1_rust = s_ans_rust.daily_tmp(case_1);
    for(const auto& v: res_1_rust) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    auto res_2_rust = s_ans_rust.daily_tmp(case_2);
    for(const auto& v: res_2_rust) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    auto res_3_rust = s_ans_rust.daily_tmp(case_3);
    for(const auto& v: res_3_rust) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    SolutionAns s_ans;
    auto res_1_ans = s_ans.dailyTemperatures(case_1);
    for(const auto& v: res_1_ans) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    auto res_2_ans = s_ans.dailyTemperatures(case_2);
    for(const auto& v: res_2_ans) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    auto res_3_ans = s_ans.dailyTemperatures(case_3);
    for(const auto& v: res_3_ans) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}
