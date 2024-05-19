#include <iostream>
#include <string>
#include <vector>

class Solution {
    public:
        // AC
        // int -> std::stringの変換を使う
        bool isPalindrome(int x) {
            if(x < 0) {
                return false;
            }
            std::string x_str = std::to_string(x);
            auto [left, right] = std::pair(0, x_str.size() - 1);
            while(left < right) {
                if(x_str[left] != x_str[right]) {
                    return false;
                }

                ++left;
                --right;
            }

            return true;
        }

        // int -> std::stringを使わないがバッファが要る
        // AC
        int isPalindrome2(int x) {
            if(x < 0) {
                return false;
            } else if(x == 0) {
                return true;
            }

            std::vector<int> digit;
            while(x > 0) {
                digit.push_back(x % 10);
                x /= 10;
            }

            auto [left, right] = std::pair(0, digit.size() - 1);
            while(left < right) {
                if(digit[left] != digit[right]) {
                    return false;
                }

                ++left;
                --right;
            }

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        // int -> std::string変換もバッファも使わない
        bool isPalindrome(int x) {
            if(x < 0) {
                return false;
            }

            long div = 1;
            while(x >= 10 * div) {
                div *= 10;
            }

            while(x != 0) {
                int right = x % 10;
                int left = x / div;

                if(left != right) {
                    return false;
                }

                x = (x % div) / 10;
                div /= 100;
            }

            return true;
        }
};

int main(void) {
    int case_1 = 121;
    // => true
    int case_2 = -121;
    // => false
    int case_3 = 10;
    // => false

    Solution s_1;

    std::cout << s_1.isPalindrome(case_1) << std::endl;
    std::cout << s_1.isPalindrome(case_2) << std::endl;
    std::cout << s_1.isPalindrome(case_3) << std::endl;

    std::cout << s_1.isPalindrome2(case_1) << std::endl;
    std::cout << s_1.isPalindrome2(case_2) << std::endl;
    std::cout << s_1.isPalindrome2(case_3) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.isPalindrome(case_1) << std::endl;
    std::cout << s_ans.isPalindrome(case_2) << std::endl;
    std::cout << s_ans.isPalindrome(case_3) << std::endl;
}
