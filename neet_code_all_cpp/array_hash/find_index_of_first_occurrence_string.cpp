#include <iostream>
#include <string>

class Solution {
    public:
        // AC
        int strStr(std::string haystack, std::string needle) {
            if(haystack.size() < needle.size()) {
                return -1;
            }

            for(auto i = 0; i < haystack.size() - needle.size() + 1; ++i) {
                int len_needle = 0;
                for(auto j = 0; j < needle.size(); ++j) {
                    if(haystack[i + j] == needle[j]) {
                        ++len_needle;
                    } else {
                        break;
                    }
                }
                if(len_needle == needle.size()) {
                    return i;
                }
            }

            return -1;
        }
};

int main(void) {
    using Case = std::pair<std::string, std::string>;
    Case case_1 = {"sadbutsad", "sad"};
    // => 0
    Case case_2 = {"leetcode", "leeto"};
    // => -1

    Solution s_1;
    std::cout << s_1.strStr(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.strStr(case_2.first, case_2.second) << std::endl;
}
