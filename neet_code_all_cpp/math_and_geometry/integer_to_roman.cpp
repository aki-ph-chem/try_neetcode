#include <iostream>
#include <string>
#include <vector>

class SolutionAns {
    public:
        // AC
        std::string intToRoman(int num) {
            std::vector<std::pair<int,std::string>> int_roman = {
                {1000, "M"},
                {900, "CM"},
                {500, "D"},
                {400, "CD"},
                {100, "C"},
                {90, "XC"},
                {50, "L"},
                {40, "XL"},
                {10, "X"},
                {9, "IX"},
                {5, "V"},
                {4, "IV"},
            };
            std::string result = "";
            for(int i = 0; i < int_roman.size(); ++i) {
                if(i + 1 < int_roman.size()) {
                    while(num >= int_roman[i].first) {
                        result += int_roman[i].second;
                        num -= int_roman[i].first;
                    }
                    if(num >= int_roman[i + 1].first) {
                        result += int_roman[i + 1].second;
                        num -= int_roman[i + 1].first;
                    }
                }
            }
            while(num >= 1) {
                result += "I";
                num -= 1;
            }

            return result;
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        std::string intToRoman(int num) {
            std::vector<std::pair<int,std::string>> int_roman = {
                {1000, "M"},
                {900, "CM"},
                {500, "D"},
                {400, "CD"},
                {100, "C"},
                {90, "XC"},
                {50, "L"},
                {40, "XL"},
                {10, "X"},
                {9, "IX"},
                {5, "V"},
                {4, "IV"},
                {1, "I"},
            };

            std::string result = "";
            for(auto& [n, r]: int_roman) {
                if(num / n > 0) {
                    auto count = num / n;
                    for(int i = 0; i < count; ++i) {
                        result += r;
                    }
                    num %= n;
                }
            }

            return result;
        }
};

int main(void) {
    int case_1 = 3749;
    // => "MMMDCCXLIX"
    int case_2 = 58;
    // => "LVIII"
    int case_3 = 1994;
    // => "MCMXCIV"
    int case_4 = 20;
    // => "XX"

    SolutionAns s_ans;

    std::cout << s_ans.intToRoman(case_1) << std::endl;
    std::cout << s_ans.intToRoman(case_2) << std::endl;
    std::cout << s_ans.intToRoman(case_3) << std::endl;
    std::cout << s_ans.intToRoman(case_4) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.intToRoman(case_1) << std::endl;
    std::cout << s_ans_rs.intToRoman(case_2) << std::endl;
    std::cout << s_ans_rs.intToRoman(case_3) << std::endl;
    std::cout << s_ans_rs.intToRoman(case_4) << std::endl;
}
