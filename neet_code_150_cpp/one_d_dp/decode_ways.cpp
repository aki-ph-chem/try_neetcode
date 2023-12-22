#include <iostream>
#include <vector>
#include <string>

// 模範解答
class SolutionAns {
    public:
        int numDecodings(std::string s) {
            if (s[0] == '0') {
                return 0;
            }

            int n = s.size();

            std::vector<int> dp(n + 1);
            dp[0] = 1;
            dp[1] = 1;

            for (int i = 2; i <= n; ++i) {
                // 1桁の場合
                int ones = stoi(s.substr(i - 1, 1));
                if(ones >= 1 && ones <= 9) {
                    dp[i] += dp[i - 1];
                }

                // ２桁の場合
                int tens = stoi(s.substr(i - 2, 2));
                if(tens >= 10 && tens <= 26) {
                    dp[i] += dp[i - 2];
                }
            }

            return dp[n];
        }
};

int main(void) {
    auto case_1 = std::string{"12"};
    auto case_2 = std::string{"226"};
    auto case_3 = std::string{"06"};
    auto case_4 = std::string{"10"};

    SolutionAns s_ans;
    std::cout << s_ans.numDecodings(case_1) << std::endl;
    std::cout << s_ans.numDecodings(case_2) << std::endl;
    std::cout << s_ans.numDecodings(case_3) << std::endl;
    std::cout << s_ans.numDecodings(case_4) << std::endl;
}
