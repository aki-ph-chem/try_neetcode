#include <iostream>

// 模範解答
class SolutionAns {
    public:
        int mySqrt(int x) {
            if(x == 0 || x == 1) {
                return x;
            }

            long long left = 0, mid = 0, right = x / 2;
            while(left <= right) {
                mid = (left + right) / 2;

                if(mid * mid == x) {
                    return mid;
                }

                if(mid * mid < x) {
                    if((mid + 1) * (mid + 1) > x) {
                        return mid;
                    }
                    left = mid + 1;
                } else {
                    if((mid - 1) * (mid - 1) < x) {
                        return mid - 1;
                    }
                    right = mid - 1;
                }             
            }

            return mid;
        }
};

int main(void) {
    int case_1 = 4;
    // => 2
    int case_2 = 8;
    // => 3
    int case_3 = 2147395599;
    // => 46339
    int case_4 = 1;
    // => 1
    int case_5 = 36;
    // => 6
    int case_6 = 9;
    // => 3

    SolutionAns s_ans;
    std::cout << s_ans.mySqrt(case_1) << std::endl;
    std::cout << s_ans.mySqrt(case_2) << std::endl;
    std::cout << s_ans.mySqrt(case_3) << std::endl;
    std::cout << s_ans.mySqrt(case_4) << std::endl;
    std::cout << s_ans.mySqrt(case_5) << std::endl;
    std::cout << s_ans.mySqrt(case_6) << std::endl;
}
