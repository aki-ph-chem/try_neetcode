#include <iostream>
#include <unordered_set>
#include <vector>
#include <string>
#include <unordered_map>

// AC
class Solution {
    public:
        std::vector<std::string> letterCombinations(std::string digits) {
            if(digits.size() < 1) {
                return std::vector<std::string>{};
            }

            std::unordered_map<int, std::string> map = 
            {
                {2, "abc"},
                {3, "def"},
                {4, "ghi"},
                {5, "jkl"},
                {6, "mno"},
                {7, "pqrs"},
                {8, "tuv"},
                {9, "wxyz"},
            };

            std::vector<std::string> result;
            std::string current;
            dfs(map, digits, 0, result, current);

            return result;
        }

    private:
        void dfs(
                std::unordered_map<int, std::string>& map,
                const std::string& digits,
                int idx,
                std::vector<std::string>& result,
                std::string& current
                ) {
            if(digits.size() <= idx) {
                result.push_back(current);
                return;
            }

            int n = (int)(digits[idx] - '0');
            for(auto& c: map[n]) {
                current.push_back(c);
                dfs(map, digits, idx + 1, result, current);
                current.pop_back();
            }
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::string> letterCombinations(std::string digits) {
            if(digits.empty()) {
                return {};
            }

            std::unordered_map<char, std::string> m = {
                {'2', "abc"},
                {'3', "def"},
                {'4', "ghi"},
                {'5', "jkl"},
                {'6', "mno"},
                {'7', "pqrs"},
                {'8', "tuv"},
                {'9', "wxyz"}
            };

            std::string current = "";
            std::vector<std::string> result;

            dfs(digits, 0, m, current, result);

            return result;
        }

    private:
        void dfs(
                std::string digits, 
                int index, 
                std::unordered_map<char, std::string>& map,
                std::string& current,
                std::vector<std::string>& result
                ) {
            if(index == digits.size()) {
                result.push_back(current);
                return;
            }

            std::string str = map[digits[index]];
            for(int i = 0; i < str.size(); ++i) {
                current.push_back(str[i]);
                dfs(digits, index + 1, map, current, result);
                current.pop_back();
            }
        }
};

void show_result(const std::vector<std::string>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << '\n';
}

int main(void) {
    std::string case_1 = "23";
    std::string case_2 = "";
    std::string case_3 = "2";

    Solution s_1;
    auto res_1 = s_1.letterCombinations(case_1);
    auto res_2 = s_1.letterCombinations(case_2);
    auto res_3 = s_1.letterCombinations(case_3);

    show_result(res_1);
    show_result(res_2);
    show_result(res_3);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.letterCombinations(case_1);
    auto res_2_ans = s_ans.letterCombinations(case_2);
    auto res_3_ans = s_ans.letterCombinations(case_3);

    show_result(res_1_ans);
    show_result(res_2_ans);
    show_result(res_3_ans);
}
