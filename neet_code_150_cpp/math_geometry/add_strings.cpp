#include <iostream>
#include <string>
#include <algorithm>

class Solution  {
    public:
        // AC
        std::string addStrings(std::string num1, std::string num2) {
            std::reverse(num1.begin(), num1.end());
            std::reverse(num2.begin(), num2.end());

            auto result = std::string{};
            auto left = std::string{};
            if(num1.size() >= num2.size()) {
                result = num1;
                left = num2;
            } else {
                result = num2;
                left = num1;
            }

            auto carry = 0;
            for(auto i = 0 ; i < left.size() ; ++i) {
                auto digit = (result[i] - '0' + left[i] - '0' + carry)%10 + '0';
                carry = (result[i] - '0' + left[i] - '0' + carry)/10;
                result[i] = digit;
            }

            auto j = left.size();
            while(carry != 0) {
                if(j < result.size()) {
                    auto digit = (result[j] - '0' + carry)%10 + '0';
                    carry = (result[j] - '0' + carry)/10;
                    result[j] = digit;
                } else {
                    result.push_back(carry + '0');
                    break;
                }
                ++j;
            }

            std::reverse(result.begin(), result.end());
            return result;
        }
};

void test_func(int n) {
    Solution s_test;
    for(int i = 0; i <= n; ++i) {
        for(int j = 0; j <= n; ++j) {
            auto i_string = std::to_string(i);
            auto j_string = std::to_string(j);

            std::cout << i_string << "+" << j_string;
            if(std::stoi(s_test.addStrings(i_string, j_string)) == i + j) {
                std::cout << ": Ok\n";
            } else {
                std::cout << ": NG\n";
            }
        }
    }
}

int main(void) {
    Solution s_1;

    std::cout << s_1.addStrings("11", "123") << "\n";
    // => 134
    std::cout << s_1.addStrings("0", "0") << "\n";
    // => 0
    std::cout << s_1.addStrings("456", "77") << "\n";
    // => 533
    std::cout << s_1.addStrings("999", "1") << "\n";
    // => 1000

    test_func(1000);
}
