#include <iostream>
#include <string>

class SolutionAns {
    public:
        bool validPalindrome(std::string s) {
            auto [i,j] = std::pair(0, (int)s.length() - 1);

            while(i < j) {
                if(s[i] == s[j]) {
                    ++i;
                    --j;;
                } else {
                    return validPalindromeUtil(s, i + 1, j) 
                        || validPalindromeUtil(s, i, j - 1);
                }
            } 

            return true;
        }
    private:
        bool validPalindromeUtil(std::string s, int i, int j) {
            while(i < j) {
                if(s[i] == s[j]) {
                    i += 1;
                    j -= 1;
                } else {
                    return false;
                }
            }

            return true;
        }
};

int main(void) {
    std::string case_1 = "aba";
    // => true
    std::string case_2 = "abca";
    // => true
    std::string case_3 = "abc";
    // => false
    std::string case_4 = "cbbcc";
    // => true
    std::string case_5 = "xdddbababeccebababddd";
    // => true

    SolutionAns s_ans;

    std::cout << s_ans.validPalindrome(case_1) << std::endl;
    std::cout << s_ans.validPalindrome(case_2) << std::endl;
    std::cout << s_ans.validPalindrome(case_3) << std::endl;
    std::cout << s_ans.validPalindrome(case_4) << std::endl;
    std::cout << s_ans.validPalindrome(case_5) << std::endl;
}
