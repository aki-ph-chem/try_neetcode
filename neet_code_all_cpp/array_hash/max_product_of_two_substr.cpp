#include <algorithm>
#include <climits>
#include <iostream>
#include <string>

// 模範解答
class SolutionAns {
    private:
        int result = INT_MIN;

        bool isPalindrome(std::string& s) {
            auto [left, right] = std::pair((int)0, (int)s.length() - 1);

            while(left < right) {
                if(s[left] != s[right]) {
                    return false;
                }
                ++left;
                --right;
            }

            return true;
        }

        void genAll(int idx, std::string& s_1, std::string& s_2, std::string& s) {
            if(idx >= s.length()) {
                if(isPalindrome(s_1) && isPalindrome(s_2)) {
                    int res_tmp = s_1.length() * s_2.length();
                    result = std::max(result, res_tmp);
                }
                return;
            }

            auto c = s[idx];

            s_1.push_back(c);
            genAll(idx + 1, s_1, s_2, s);
            s_1.pop_back();

            s_2.push_back(c);
            genAll(idx + 1, s_1, s_2, s);
            s_2.pop_back();

            genAll(idx + 1, s_1, s_2, s);
        }

    public:
        int maxProduct(std::string s) {
            std::string s_1 = "", s_2 = "";
            int idx = 0;

            genAll(idx, s_1, s_2, s);

            return result;
        }
};


int main(void) {
    std::string case_1 = "leetcodecom";
    // => 9
    std::string case_2 = "bb";
    // => 1
    std::string case_3 = "accbcaxxcxx";
    // => 25

    SolutionAns s_ans_1;
    std::cout << s_ans_1.maxProduct(case_1) << std::endl;

    SolutionAns s_ans_2;
    std::cout << s_ans_2.maxProduct(case_2) << std::endl;

    SolutionAns s_ans_3;
    std::cout << s_ans_3.maxProduct(case_3) << std::endl;
}
