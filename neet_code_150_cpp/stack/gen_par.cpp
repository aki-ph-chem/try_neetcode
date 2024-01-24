#include <vector>
#include <string>
#include <iostream>

// Rustの模範解答より
class SolutionAnsRust{
    public:
        std::vector<std::string> gen_par(int n) {
            std::vector<std::string> res;
            backtrack(res, "", n, n);
            return res;
        }
    private:
        // open, closeを両方ともnからスタートして0に等しくなるまで減らしながら再帰
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

// 模範解答
class SolutionAns {
    public:
        std::vector<std::string> generateParenthesis(int n) {
            std::vector<std::string> result;
            generate(n, 0, 0, "", result);
            return result;
        }
    private:
        // open, closeを両方とも0からスタートしてnに等しくなるまで増やしながら再帰
        void generate(int n, int open, int close, std::string str, std::vector<std::string>& result) {
            if (open == n && close == n) {
                result.push_back(str);
                return;
            }
            if (open < n) {
                generate(n, open + 1, close, str + '(', result);
            }
            if (open > close) {
                generate(n, open, close + 1, str + ')', result);
            }
        }
};

int main() {
    SolutionAnsRust s_1;

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


    SolutionAns s_ans;

    auto res_ans_1 = s_ans.generateParenthesis(3);
    for(const auto& s: res_ans_1) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    auto res_ans_2 = s_ans.generateParenthesis(2);
    for(const auto& s: res_ans_2) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    auto res_ans_3 = s_ans.generateParenthesis(1);
    for(const auto& s: res_3) {
        std::cout << s << " ";
    }
    std::cout << std::endl;
}
