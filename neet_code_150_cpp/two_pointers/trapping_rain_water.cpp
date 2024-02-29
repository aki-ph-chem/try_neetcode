#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        int trap(std::vector<int>& height) {
            auto [i,j] = std::pair(0, height.size() - 1);
            auto [maxLeft, maxRight] = std::pair(height[i], height[j]);

            int result = 0;
            while(i < j) {
                if(maxLeft <= maxRight) {
                    ++i;
                    maxLeft = std::max(maxLeft, height[i]);
                    result += maxLeft - height[i];
                } else {
                    --j;
                    maxRight = std::max(maxRight, height[j]);
                    result += maxRight - height[j];
                }
            }

            return result;
        }
};

int main(void) {
    std::vector<int> case_1 = {0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1};
    // => 6
    std::vector<int> case_2 = {4, 2, 0, 3, 2, 5};
    // => 9
    std::vector<int> case_3 = {0, 1, 0, 0, 1, 0, 1};
    // => 3

    SolutionAns s_ans;
    std::cout << s_ans.trap(case_1) << std::endl;
    std::cout << s_ans.trap(case_2) << std::endl;
    std::cout << s_ans.trap(case_3) << std::endl;
}
