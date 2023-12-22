#include <cstdint>
#include <iostream>
#include <string>

// 模範解答
class SolutionAns {
    public:
        int countSubstring(std::string s) {
            int result = 0;

            for (int i = 0; i < s.size(); ++i) {
                middleOut(s, i ,i ,result);
                middleOut(s, i ,i + 1 ,result);
            }

            return result;
        }

    private:
        void middleOut(std::string s, int i, int j, int& result) {
            while(i >= 0 && j < s.size() && s[i] == s[j]) {
                ++result;
                --i;
                ++j;
            }
        }

    public:
        // Rustの模範解答より
        int countSubstring2(std::string s) {
            int count = 0, lenght = s.size();

            for(int i = 0;i < lenght; ++i) {
                // odd length
                int l = i, r = i;
                while(l >= 0 && r < lenght && s[l] == s[r]) {
                    ++count;
                    --l;
                    ++r;
                }

                // even lenght
                l = i, r = i + 1;
                while(l >= 0 && r < lenght && s[l] == s[r]) {
                    ++count;
                    --l;
                    ++r;
                }
            }
            return count;
        }
};

int main(void) {
    auto case_1 = std::string{"abc"};
    auto case_2 = std::string{"aaa"};

    SolutionAns s_ans;
    std::cout << s_ans.countSubstring(case_1) << std::endl;
    std::cout << s_ans.countSubstring(case_2) << std::endl;
}
