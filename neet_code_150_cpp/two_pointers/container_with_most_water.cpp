#include <algorithm>
#include <iostream>
#include <vector>

// Rustの模範解答より
class Solution {
    public:
        int max_area(std::vector<int>& height) {
            int max_area = 0;
            int l = 0, r = height.size() - 1;

            while(l < r) {
                int area = (r - l) * std::min(height[l], height[r]);
                max_area = std::max(area, max_area);

                if(height[l] > height[r]) {
                    --r;
                } else {
                    ++l;
                }
            }

            return max_area;
        }
};

// C++の模範解答
class SolutionAns {
public:
    int maxArea(std::vector<int>& height) {
        int i = 0;
        int j = height.size() - 1;
        
        int curr = 0;
        int result = 0;
        
        while (i < j) {
            curr = (j - i) * std::min(height[i], height[j]);
            result = std::max(result, curr);
            
            if (height[i] <= height[j]) {
                i++;
            } else {
                j--;
            }
        }
        
        return result;
    }
};
int main(void) {
    auto case_1 = std::vector{1, 8, 6, 2, 5, 4, 8, 3, 7};
    auto case_2 = std::vector{1, 1};
    auto case_3 = std::vector{1, 1, 1, 1};
    auto case_4 = std::vector{3, 1, 1, 3};

    Solution s_1;
    std::cout << s_1.max_area(case_1) << std::endl;
    std::cout << s_1.max_area(case_2) << std::endl;
    std::cout << s_1.max_area(case_3) << std::endl;
    std::cout << s_1.max_area(case_4) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.maxArea(case_1) << std::endl;
    std::cout << s_ans.maxArea(case_2) << std::endl;
    std::cout << s_ans.maxArea(case_3) << std::endl;
    std::cout << s_ans.maxArea(case_4) << std::endl;
}
