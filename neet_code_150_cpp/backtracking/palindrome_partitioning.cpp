#include <iostream>
#include <string>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<std::string>> partition(const std::string& s) {
            std::vector<std::string> current;
            std::vector<std::vector<std::string>> result;
            dfs(s, 0, current, result);

            return result;
        }

    private:
        void dfs(
                const std::string& s, 
                int start, 
                std::vector<std::string>& current, 
                std::vector<std::vector<std::string>>& result
                ) {
            if(start == s.size()) {
                result.push_back(current);
                return;
            }

            for(int i = start; i < s.size(); ++i) {
                if(isPalindrome(s, start, i)) {
                    auto str = s.substr(start, i - start + 1);
                    current.push_back(str);

                    dfs(s, i + 1, current, result);
                    current.pop_back();
                }
            }
        }

        bool isPalindrome(const std::string& s, int left, int right) {
            while(left < right) {
                if(s[left] != s[right]) {
                    return false;
                }
                ++left;
                --right;
            }

            return true;
        }
};

void show_result(const std::vector<std::vector<std::string>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    auto case_1 = std::string{"aab"};
    // => [["a", "a", "b"], ["aa", "b"]]
    auto case_2 = std::string{"a"};
    // => [["a"]]

    SolutionAns s_ans;

    auto res_1 = s_ans.partition(case_1);
    auto res_2 = s_ans.partition(case_2);
    show_result(res_1);
    show_result(res_2);
}
