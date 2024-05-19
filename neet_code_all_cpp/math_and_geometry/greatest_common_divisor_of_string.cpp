#include <iostream>
#include <string>
#include <utility>

// 模範解答
class SolutionAns {
    public:
        std::string gcdOfStrings(std::string str1, std::string str2) {
            std::string shortest, longest;
            if(str1.length() < str2.length()) {
                shortest = str1;
                longest = str2;
            } else {
                shortest = str2;
                longest = str1;
            }

            auto result = std::string{};
            auto [len_short, len_long] = std::pair(shortest.size(), longest.size());

            for(int i = len_short; i > 0; --i) {
                if(len_long % i != 0 || len_short % i != 0) {
                    continue;
                }

                for(int j = 0; j < len_long; ++j) {
                    auto [ptr_first, ptr_second] = std::pair(j % i, j % len_short);
                    if(shortest[ptr_first] != longest[j] || shortest[ptr_second] != longest[j]) {
                        result = ""; 
                        break;
                    }

                    if(ptr_first == j) {
                        result += longest[j];
                    }
                }

                if(result != "") {
                    return result;
                }
            }

            return "";
        }
};

int main(void) {
    std::pair<std::string, std::string> case_1 = {"ABCABC", "ABC"};
    // => "ABC"
    std::pair<std::string, std::string> case_2 = {"ABABAB", "ABAB"};
    // => "AB"
    std::pair<std::string, std::string> case_3 = {"LEET", "CODE"};
    // => ""

    SolutionAns s_ans;

    std::cout << s_ans.gcdOfStrings(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.gcdOfStrings(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.gcdOfStrings(case_3.first, case_3.second) << std::endl;
}
