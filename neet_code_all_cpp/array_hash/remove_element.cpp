#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
public:
    int removeElement(std::vector<int>& nums, int val) {
        int n = (int)nums.size();
        int count=0;
        for(int i = 0;i < n; ++i)
        {
            if(nums[i]!= val)
            {
                std::swap(nums[i], nums[count]);
                count++;
            }
        }
        
        return count;
    }

    // AC
    int removeElement2(std::vector<int>& nums, int val) {
        int k = 0;
        for(int i = 0; i < nums.size(); ++i) {
            if(nums[i] != val) {
                nums[k] = nums[i];
                ++k;
            }
        }

        return k;
    }

    // AC
    int removeElement3(std::vector<int>& nums, int val) {
        int k = 0;
        int i = 0;
        while(i < nums.size()) {
            if(nums[i] == val) {
                ++i;
            } else {
                nums[k] = nums[i];
                ++k;
                ++i;
            }
        }

        return k;
    }
};

void show_resutl(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{3,2,2,3}, 3};
    Case case_2 = {{0,1,2,2,3,0,4,2}, 2};

    SolutionAns s_ans;

    auto res_1 = case_1.first;
    std::cout << s_ans.removeElement(res_1, case_1.second) << std::endl;
    show_resutl(res_1);

    auto res_2 = case_2.first;
    std::cout << s_ans.removeElement(res_2, case_2.second) << std::endl;
    show_resutl(res_2);

    res_1 = case_1.first;
    std::cout << s_ans.removeElement2(res_1, case_1.second) << std::endl;
    show_resutl(res_1);

    res_2 = case_2.first;
    std::cout << s_ans.removeElement2(res_2, case_2.second) << std::endl;
    show_resutl(res_2);

    res_1 = case_1.first;
    std::cout << s_ans.removeElement3(res_1, case_1.second) << std::endl;
    show_resutl(res_1);

    res_2 = case_2.first;
    std::cout << s_ans.removeElement3(res_2, case_2.second) << std::endl;
    show_resutl(res_2);
}
