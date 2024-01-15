#include <iostream>
#include <string>
#include <vector>

class SolutionAns {
    public:
        bool is_palindrome(std::string s) {
            int i = 0;
            int j = s.size() - 1;

            while (i < j) {
                while (!isalnum(s[i]) && i < j) {
                    i++;
                }
                while (!isalnum(s[j]) && i < j) {
                    j--;
                }
                if (tolower(s[i]) != tolower(s[j])) {
                    return false;
                }
                i++;
                j--;
            }
            return true;
        }
};

int main(void){
    auto case_1 = std::string{"A man, a plan, a canal: Panama"};
    auto case_2 = std::string{"race a car"};
    auto case_3 = std::string{""}; 

    SolutionAns s_1;
    std::cout << s_1.is_palindrome(case_1) << std::endl;
    std::cout << s_1.is_palindrome(case_2) << std::endl;
    std::cout << s_1.is_palindrome(case_3) << std::endl;
}
