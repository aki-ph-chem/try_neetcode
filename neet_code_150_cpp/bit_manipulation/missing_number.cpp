#include <iostream>
#include <vector>

class SolutionAns {
public:
    int missingNumber(std::vector<int>& nums) {
        int n = nums.size();
        int result = n;
        for (int i = 0; i < n; i++) {
            result ^= i ^ nums[i];
        }
        return result;
    }
};

int main(void) {
    auto case_1 = std::vector{3, 0, 1};
    // => 2
    auto case_2 = std::vector{0, 1};
    // => 2
    auto case_3 = std::vector{9, 6, 4, 2, 3, 5, 7, 0, 1};
    // => 8

    SolutionAns s_ans;
    std::cout << s_ans.missingNumber(case_1) << std::endl;
    std::cout << s_ans.missingNumber(case_2) << std::endl;
    std::cout << s_ans.missingNumber(case_3) << std::endl;
}
