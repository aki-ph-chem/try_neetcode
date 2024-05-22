#include <iostream>
#include <string>

class SolutionAns {
    public:
        std::string convert(std::string s, int numRows) {
            if(numRows == 1) {
                return s;
            }

            std::string result = "";
            for(int row = 0; row < numRows; ++row) {
                int increment = 2 * (numRows - 1);
                for(int i = row; i < s.length(); i += increment) {
                    result += s[i];
                    // ??
                    if(row > 0 && row < numRows - 1 && i + increment - 2 * row < s.length()) {
                        result += s[i + increment - 2 * row];
                    }
                }
            }

            return result;
        }
};

int main(void) {
    std::pair<std::string,int> case_1 = {"PAYPALISHIRING", 3};
    // => "PAHNAPLSIIGYIR"
    std::pair<std::string,int> case_2 = {"PAYPALISHIRING", 4};
    // => "PINALSIGYAHRPI"
    std::pair<std::string,int> case_3 = {"A", 1};
    // => "A"

    SolutionAns s_ans;

    std::cout << s_ans.convert(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.convert(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.convert(case_3.first, case_3.second) << std::endl;
}
