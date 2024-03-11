#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        int removeDuplicates(std::vector<int>& nums) {
            auto [k,l,count] = std::tuple(2, 1, 1);

            for(int r = 1; r < nums.size(); ++r) {
                if(nums[r] == nums[r - 1]) {
                    if(count < k) {
                        nums[l++] = nums[r];
                        ++count;
                    }
                } else {
                    count = 1;
                    nums[l++] = nums[r];
                }
            }

            return l;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }

    std::cout << std::endl;
}

int main(void) {
    std::vector<int> case_1 = {1, 1, 1, 2, 2, 3};
    // => [1,1,2,2,3,_]
    std::vector<int> case_2 = {0, 0, 1, 1, 1, 1, 2, 3, 3};
    // => [0,0,1,1,2,3,3,_,_]

    SolutionAns s_ans;

    auto res_1 = case_1;
    std::cout << s_ans.removeDuplicates(res_1) << std::endl;
    show_result(res_1);
}
