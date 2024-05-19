#include <iostream>
#include <string>
#include <algorithm>

class SolutionAnsPython {
    public:
        std::string convertToTitle(int columnNumber) {
            auto result = std::string{};
            while(columnNumber > 0) {
                auto tmp = (columnNumber - 1) % 26;
                result.push_back('A' + tmp);
                columnNumber = (columnNumber - 1) / 26;
            }

            std::reverse(result.begin(), result.end());
            return result;
        }
};

int main(void) {
    int case_1 = 1;
    // "A"
    int case_2 = 28;
    // "AB"
    int case_3 = 701;
    // "ZY"

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.convertToTitle(case_1) << std::endl;
    std::cout << s_ans_py.convertToTitle(case_2) << std::endl;
    std::cout << s_ans_py.convertToTitle(case_3) << std::endl;
}
