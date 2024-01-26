#include <iostream>
#include <climits>

class SolutionAns {
    public:
        int reverse(int x) {
            int rev = 0;
            while (x != 0) {
                int temp = x % 10;
                x /= 10;
                if (rev > INT_MAX / 10 || (rev == INT_MAX / 10 && temp > 7)) {
                    return 0;
                }
                if (rev < INT_MIN / 10 || (rev == INT_MIN / 10 && temp < -8)) {
                    return 0;
                }
                rev = rev * 10 + temp;
            }
            return rev;
        }
};

int main(void) {
    SolutionAns s_ans;

    int case_1 = 123;
    int case_2 = -123;
    int case_3 = 120;

    std::cout <<s_ans.reverse(case_1) << std::endl;
    std::cout <<s_ans.reverse(case_2) << std::endl;
    std::cout <<s_ans.reverse(case_3) << std::endl;
}
