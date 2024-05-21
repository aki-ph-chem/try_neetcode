#include <iostream>
#include <string>
#include <unordered_map>
#include <array>

class SolutionAns {
    public:
        // AC
        int romanToInt(std::string s) {
            /*
            std::unordered_map<char, int> map_1 = {
                {'I', 1},
                {'V', 2},
                {'X', 3},
                {'L', 4},
                {'C', 5},
                {'D', 6},
                {'M', 7},
            };
            auto prec = [&](char c) {
                if(!map_1.count(c)) {
                    return -1;
                }

                return map_1[c];
            };
            std::unordered_map<char, int> map_2 = {
                {'I', 1},
                {'V', 5},
                {'X', 10},
                {'L', 50},
                {'C', 100},
                {'D', 500},
                {'M', 1000},
            };
            auto val = [&](char c) {
                if(!map_2.count(c)) {
                    return -1;
                }

                return map_2[c];
            };
            */

            auto prec_sw = [](char c) {
                switch (c) {
                    case 'I':
                        return 1;
                    case 'V':
                        return 2;
                    case 'X':
                        return 3;
                    case 'L':
                        return 4;
                    case 'C':
                        return 5;
                    case 'D':
                        return 6;
                    case 'M':
                        return 7;
                    default:
                        return -1;
                }
            };
            auto val_sw = [](char c){
                switch (c) {
                    case 'I':
                        return 1;
                    case 'V':
                        return 5;
                    case 'X':
                        return 10;
                    case 'L':
                        return 50;
                    case 'C':
                        return 100;
                    case 'D':
                        return 500;
                    case 'M':
                        return 1000;
                    default:
                        return -1;
                }
            };

            int result = 0;
            for(int i = 0; i < s.length(); ++i) {
                if(prec_sw(s[i]) >= prec_sw(s[i + 1])) {
                    result += val_sw(s[i]);
                } else if (prec_sw(s[i]) < prec_sw(s[i + 1])) {
                    result = result - val_sw(s[i]) + val_sw(s[i + 1]);
                    ++i;
                }
            }

            return result;
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        int romanToInt(std::string s) {
            auto to_value = [](char c) {
                switch(c) {
                    case 'I':
                        return 1;
                    case 'V':
                        return 5;
                    case 'X':
                        return 10;
                    case 'L':
                        return 50;
                    case 'C':
                        return 100;
                    case 'D':
                        return 500;
                    case 'M':
                        return 1000;
                    default:
                        return 0;
                }
            };
            int result = 0;
            for(int i = 0; i < s.length(); ++i) {
                if(i + 1 < s.length() && to_value(s[i]) < to_value(s[i + 1])) {
                    result -= to_value(s[i]);
                } else {
                    result += to_value(s[i]);
                }
            }

            return result;
        }
};

int main(void) {
    std::string case_1 = "III";
    // => 3
    std::string case_2 = "LVIII";
    // => 58
    std::string case_3 = "MCMXCIV";

    SolutionAns s_ans;

    std::cout << s_ans.romanToInt(case_1) << std::endl;
    std::cout << s_ans.romanToInt(case_2) << std::endl;
    std::cout << s_ans.romanToInt(case_3) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.romanToInt(case_1) << std::endl;
    std::cout << s_ans_rs.romanToInt(case_2) << std::endl;
    std::cout << s_ans_rs.romanToInt(case_3) << std::endl;
}
