#include <iostream>
#include <string>
#include <vector>

// Rustの模範解答より
class Solution {
    public:
        std::string longest_plindrome(std::string s) {
            int left = 0, right = 0, length = s.size();

            if(length == 1) {
                return std::string{s[0]};
            }

            for(int i = 0; i < length; ++i) {
                // odd length
                int l = i, r = i;
while(l >= 0 && r < length && s[l] == s[r]) {
                    // r - l: 新しい部分文字列の長さが right - left: 前の部分文字列の長さ
                    // より大きければ更新
                    if (r - l > right - left) {
                        left = l;
                        right = r;
                    }
                    --l;
                    ++r;
                }

                // even length
                l = i;
                r = i + 1;
                while(l >= 0 && r < length && s[l] == s[r]) {
                    // r - l: 新しい部分文字列の長さが right - left: 前の部分文字列の長さ
                    // より大きければ更新
                    if (r - l > right - left) {
                        left = l;
                        right = r;
                    }
                    --l;
                    ++r;
                }
            }
            return s.substr(left, right - left + 1);
        }
};

// 模範解答
class SolutionAns {
    public:
        std::string longest_plindrome(std::string s) {
            int maxStart = 0;
            int maxLength = 1;

            for (int i = 0; i < s.size() - 1; i++) {
                middleOut(s, i, i, maxStart, maxLength);
                middleOut(s, i, i + 1, maxStart, maxLength);
            }

            return s.substr(maxStart, maxLength);
        }

    private:
        void middleOut(std::string s, int i, int j, int& maxStart, int& maxLength) {
            while (i >= 0 && j <= s.size() - 1 && s[i] == s[j]) {
                i--;
                j++;
            }
            // j - i - 1 がこれまでの最大の回文部分文字列より大きければ更新
            if (j - i - 1 > maxLength) {
                maxStart = i + 1;
                maxLength = j - i - 1;
            }
        }
};

int main(void) {
    auto case_1 = std::string{"babad"};
    // "bab" or "aba"
    auto case_2 = std::string{"cbbd"};
    // "bb"

    Solution s_1;
    std::cout << s_1.longest_plindrome(case_1) << std::endl;
    std::cout << s_1.longest_plindrome(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.longest_plindrome(case_1) << std::endl;
    std::cout << s_ans.longest_plindrome(case_2) << std::endl;
}
