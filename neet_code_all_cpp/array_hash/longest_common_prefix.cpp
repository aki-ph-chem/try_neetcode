#include <climits>
#include <iostream>
#include <string>
#include <vector>

class Solution {
    public:
        // AC 
        std::string longestCommonPrefix(std::vector<std::string>& strs) {
            int w_len_min = INT_MAX;
            for(auto& word: strs) {
                w_len_min = std::min(w_len_min, (int)word.size());
            }

            std::string result;

            for(int w = 0; w < w_len_min; ++w) {
                auto c_tmp = strs[0][w];
                int i = 1;
                while(i < strs.size()) {
                    if(c_tmp != strs[i][w]) {
                        break;
                    }
                    ++i;
                }

                if(i >= strs.size()) {
                    result.push_back(c_tmp);
                } else {
                    break;
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::string longestCommonPrefix(std::vector<std::string>& strs) {
            std::string result = strs[0];
            int charIndex = 0;

            long maxCharIndex = strs[0].length();
            for(auto& word: strs) {
                maxCharIndex = std::min(maxCharIndex, (long)word.size());
            }

            while(charIndex < maxCharIndex) {
                auto prevChar = strs[0][charIndex];
                for(int i = 1; i < strs.size(); ++i) {
                    if(prevChar == strs[i][charIndex]) {
                        continue;
                    }

                    return result.substr(0, charIndex);
                }
                ++charIndex;
                result += prevChar;
            }

            return result.substr(0, charIndex);
        }
};

int main(void) {
    using Case = std::vector<std::string>;
    Case case_1 = {"flower","flow","flight"};
    Case case_2 = {"dog","racecar","car"};

    Solution s_1;
    std::cout << s_1.longestCommonPrefix(case_1) << std::endl;
    std::cout << s_1.longestCommonPrefix(case_2) << std::endl;
}
