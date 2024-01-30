#include <cwctype>
#include <iostream>
#include <deque>
#include <vector>

class SolutionAns {
public:
    std::vector<int> maxSlidingWindow(std::vector<int>& nums, int k) {
        std::deque<int> dq;
        std::vector<int> result;
        
        int i = 0;
        int j = 0;
        
        while (j < nums.size()) {
            while (!dq.empty() && nums[dq.back()] < nums[j]) {
                dq.pop_back();
            }
            dq.push_back(j);
            
            if (i > dq.front()) {
                dq.pop_front();
            }
            
            if (j + 1 >= k) {
                result.push_back(nums[dq.front()]);
                i++;
            }
            j++;
        }
        
        return result;
    }
};

void show_vec(std::vector<int>& vec) {
    for(auto& v: vec) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto case_1 = std::pair(std::vector{1, 3, -1, -3, 5, 3, 6, 7}, 3);
    auto case_2 = std::pair(std::vector{1}, 1);

    SolutionAns s_ans;

    auto res_1 = s_ans.maxSlidingWindow(case_1.first, case_1.second);
    show_vec(res_1);

    auto res_2 = s_ans.maxSlidingWindow(case_2.first, case_2.second);
    show_vec(res_2);
}
