#include <iostream>
#include <string>

// 模範解答(Python)は全く同じだった
class Solution {
    public:
        // AC
        std::string removingStars(std::string s) {
            std::string result;
            for(auto& c: s) {
                if(c == '*') {
                    result.pop_back();
                } else {
                    result.push_back(c);
                }
            }

            return result;
        }
};

int main(void) {
    std::string case_1 = "leet**cod*e";
    std::string case_2 = "erase*****";

    Solution s_1;

    std::cout << s_1.removingStars(case_1) << std::endl;
    std::cout << s_1.removingStars(case_2) << std::endl;
}
