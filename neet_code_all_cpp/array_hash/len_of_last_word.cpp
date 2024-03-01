#include <iostream>
#include <string>
#include <vector>

// AC
class Solution {
    public:
        int lengthOfLastWord(std::string s) {
            auto n = (int)s.size();
            auto [r, w] = std::pair(0, 0);
            std::vector<int> w_list;

            while(r < n) {
                if(s[r] == ' ') {
                    ++r;
                } else {
                    while(r < n && s[r] != ' ') {
                        ++r;
                        ++w;
                    }

                    w_list.push_back(w);
                    w = 0;
                }
            }

            return w_list.back();
        }
};

// 模範解答
// これが一番シンプルな解法かも
class SolutionAns {
    public:
        int lengthOfLastWord(std::string s) {
            auto n = (int)s.size();

            auto ptr = n - 1;
            while(ptr >= 0 && s[ptr] == ' ')--ptr;

            auto len = 0;
            while(ptr >=0 && s[ptr--] != ' ')++len;

            return len;
        }
};

int main(void) {
    std::string case_1 = "Hello World"; 
    // => 5
    std::string case_2 = "   fly me   to   the moon  "; 
    // => 4
    std::string case_3 = "luffy is still joyboy"; 
    // => 6

    Solution s_1;
    std::cout << s_1.lengthOfLastWord(case_1) << std::endl;
    std::cout << s_1.lengthOfLastWord(case_2) << std::endl;
    std::cout << s_1.lengthOfLastWord(case_3) << std::endl;
}
