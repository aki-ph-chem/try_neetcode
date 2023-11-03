#include <vector>
#include <string>
#include <iostream>

class Solution{
    public:
        std::vector<std::string> gen_par(int n) {
            std::vector<std::string> res;
            backtrack(res, "", n, n);
            return res;
        }
    private:
        void backtrack(
                std::vector<std::string>& res, 
                std::string s,
                int open,
                int close) {
            if (open == 0 && close == 0) {
                res.push_back(s);
                return;
            }

            if (open == close) {
                backtrack(res, s + "(", open - 1, close);
            } else {
                if (open > 0) {
                    backtrack(res, s + "(", open - 1, close);
                }
                if (close > 0) {
                    backtrack(res, s + ")", open, close - 1);
                }
            }
        }
};

int main() {
    Solution s_1;
    auto res_1 = s_1.gen_par(3);
    for(const auto& s: res_1) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    auto res_2 = s_1.gen_par(2);
    for(const auto& s: res_2) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    auto res_3 = s_1.gen_par(1);
    for(const auto& s: res_3) {
        std::cout << s << " ";
    }
    std::cout << std::endl;
}
