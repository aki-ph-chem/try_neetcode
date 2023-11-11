#include <cstddef>
#include <iostream>
#include <string>

class SolutionAns {
    public:
        std::string multiply(std::string num1, std::string num2) {
            int m = num1.size();
            int n = num2.size();

            std::string result(m + n, '0');

            // 筆算を実装
            for(int i = m - 1; i >= 0 ; --i) {
                for(int j = n - 1; j >=0; --j) {
                   auto sum = (num1[i] - '0') * (num2[j] - '0') + (result[i + j + 1] - '0'); 
                   result[i + j + 1 ] = sum % 10 + '0';
                   result[i + j] += sum / 10;
                }
            }

            for(int i = 0; i < m + n; ++i) {
                if( result[i] != '0' ) {
                    return result.substr(i);
                }
            }
            return "0";

        }
};

int main(void) {
    auto case_1 = std::make_pair("2", "3");
    auto case_2 = std::make_pair("123", "456");
    auto case_3 = std::make_pair("12", "4");

    SolutionAns s_1;
    std::cout << s_1.multiply(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.multiply(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.multiply(case_3.first, case_3.second) << std::endl;
}
