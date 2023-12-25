#include <iostream>
#include <vector>
#include <string>

// 模範解答
class SolutionAns {
    public:
        int longestCommonSubsequence(std::string text1, std::string text2) {
            int m = text1.size();
            int n = text2.size();

            std::vector<std::vector<int>> dp(m + 1, std::vector<int>(n + 1));

            // m -1, n - 1からスタートしてi, jを減らしながら0, 0を目指す
            for (int i = m - 1; i >= 0; i--) {
                for (int j = n - 1; j >= 0; j--) {
                    if (text1[i] == text2[j]) {
                        dp[i][j] = 1 + dp[i + 1][j + 1];
                    } else {
                        dp[i][j] = std::max(dp[i + 1][j], dp[i][j + 1]);
                    }
                }
            }

            return dp[0][0];
        }

        // Rustの実装より
        int longestCommonSubsequence2(std::string text1, std::string text2) {
            auto l_1 = text1.size();
            auto l_2 = text2.size();

            // dp[i][j]: text1, text2 の i - 1, j - 1 文字目までのスコア 
            std::vector<std::vector<int>> dp(l_1 + 1, std::vector(l_2 + 1, 0));

            // 1, 1 からスタートして,i ,j を増やしながら l_1, l_2 を目指す
            for(int i = 1; i < l_1 + 1; ++i) {
                for(int j = 1; j < l_2 + 1; ++j) {
                    // i - 1 文字目とj - 1 文字目が一致したらスコアをプラス
                    if(text1[i - 1] == text2[j - 1]) {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                    } else {
                        // i - 1 文字目を削除するか、j - 1文字目を削除するか
                        dp[i][j] = std::max(dp[i - 1][j], dp[i][j - 1]);
                    }
                }
            }

            return dp[l_1][l_2];
        }
};

int main(void) {
    auto case_1 = std::pair(std::string{"abced"}, std::string{"ace"});
    // 3
    auto case_2 = std::pair(std::string{"abc"}, std::string{"abc"});
    // 3
    auto case_3 = std::pair(std::string{"abc"}, std::string{"def"});
    // 0

    SolutionAns s_ans;

    std::cout << s_ans.longestCommonSubsequence2(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.longestCommonSubsequence2(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.longestCommonSubsequence2(case_3.first, case_3.second) << std::endl;
}
