#include <cwctype>
#include <iostream>
#include <string>

// 模範解答
class SolutionAns {
    public:
        std::string addBinary(std::string a, std::string b) {
            std::string res;
            int maxLen = a.size() > b.size() ? a.size() : b.size();
            unsigned int carry = 0;

            for(int i = 0; i < maxLen; ++i) {
                unsigned int bitA = i < a.size() ? a[a.size() -i - 1] - '0': 0;
                unsigned int bitB = i < b.size() ? b[b.size() -i - 1] - '0': 0;

                unsigned int total = bitA + bitB + carry;
                char sum = '0' + total % 2;
                carry = total / 2;

                res.insert(0, 1, sum);
            }

            if(carry) {
                res.insert(0, 1, '1');
            }

            return res;
        }
};

int main(void) {
    using Case = std::pair<std::string, std::string>;

    Case case_1 = {"11", "1"};
    // => "100"
    Case case_2 = {"1010", "1011"};
    // => "10101"

    SolutionAns s_ans;
    std::cout << s_ans.addBinary(case_1.first, case_1.second) <<  std::endl;
    std::cout << s_ans.addBinary(case_2.first, case_2.second) <<  std::endl;
}
