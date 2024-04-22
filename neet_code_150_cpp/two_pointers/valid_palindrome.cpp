#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>

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

        // AC
        // 文字がアルファベットもしくは数字かの判定
        // 大文字->小文字変換
        // まで自分で実装するならこう書く
        bool is_palindrome_2(std::string s) {
            int i = 0;
            int j = s.size() - 1;

            auto is_alphanumeric = [](auto c) {
                char num_start = '0', num_end = '9';
                char alphabet_start = 'a', alphabet_end = 'z';
                char alphabet_capital_start = 'A', alphabet_capital_end = 'Z';

                if((num_start <= c && c <= num_end) || (alphabet_start <= c && c <= alphabet_end) 
                        || (alphabet_capital_start <= c && c <= alphabet_capital_end)) {
                    return true;
                } 

                return false;
            };

            auto to_lowercase = [](auto c) {
                char alphabet_start = 'A', alphabet_end = 'Z';

                if(alphabet_start <= c && c <= alphabet_end) {
                    return (char)(c - ('A' - 'a'));
                } 

                return c;
            };

            while (i < j) {
                while (!is_alphanumeric(s[i]) && i < j) {
                    i++;
                }
                while (!is_alphanumeric(s[j]) && i < j) {
                    j--;
                }
                if (to_lowercase(s[i]) != to_lowercase(s[j])) {
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

    std::cout << s_1.is_palindrome_2(case_1) << std::endl;
    std::cout << s_1.is_palindrome_2(case_2) << std::endl;
    std::cout << s_1.is_palindrome_2(case_3) << std::endl;
}
