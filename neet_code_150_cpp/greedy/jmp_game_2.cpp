#include <iostream>
#include <vector>

// 初見では解けなかった
// 模範解答 
class SolutionAns {
    public:
        int jump(std::vector<int>& nums) {
            int n = nums.size();
            int result = 0;

            int i = 0;
            while (i < n - 1) {
                if (i + nums[i] >= n - 1) {
                    result++;
                    break;
                }
                int maxIndex = i + 1;
                int maxValue = 0;
                for (int j = i + 1; j < i + 1 + nums[i]; j++) {
                    if (j + nums[j] > maxValue) {
                        maxIndex = j;
                        maxValue = j + nums[j];
                    }
                }
                i = maxIndex;
                result++;
            }

            return result;
        }
};

int main(void) {
    auto case_1 = std::vector{2, 3, 1, 1, 4};
    auto case_2 = std::vector{3, 2, 1, 0, 4};
    auto case_3 = std::vector{2, 0};
    auto case_4 = std::vector{2,5,0,0};

    SolutionAns s_ans;

    std::cout << s_ans.jump(case_1) << std::endl;
    std::cout << s_ans.jump(case_2) << std::endl;
    std::cout << s_ans.jump(case_3) << std::endl;
    std::cout << s_ans.jump(case_4) << std::endl;
}
