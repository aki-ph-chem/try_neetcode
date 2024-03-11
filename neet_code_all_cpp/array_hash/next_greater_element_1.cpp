#include <iostream>
#include <vector>
#include <stack>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        std::vector<int> nextGreaterElement(std::vector<int>& nums1, std::vector<int>& nums2) {
            // {value: index}なMap
            std::unordered_map<int, int> map; 
            for(int idx = 0; idx < nums1.size(); ++idx) {
                map[nums1[idx]] = idx;
            }

            std::vector<int> result(nums1.size(), -1);
            std::stack<int> stack;

            for(auto& current: nums2){ 

                while(stack.size() && current > stack.top()) {
                    auto val = stack.top();
                    stack.pop();
                    auto idx = map[val];
                    result[idx] = current;
                }

                if(map.count(current)) {
                    stack.push(current);
                }
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::pair<std::vector<int>, std::vector<int>>;

    Case case_1 = {{4, 1, 2}, {1, 3, 4, 2}};
    // => [-1,3,-1]
    Case case_2 = {{2, 4}, {1, 2, 3, 4}};
    // => [3,-1]
    Case case_4 = {{1, 3, 5, 2, 4}, {6, 5, 4, 3, 2, 1, 7}};
    // => [7,7,7,7,7]

    /*
    SolutionAnsRust s_ans_rs;

    auto res_1 = s_ans_rs.nextGreaterElement(case_1.first, case_1.second);
    show_result(res_1);
    */

    SolutionAns s_ans;
    auto res_1_ans = s_ans.nextGreaterElement(case_1.first, case_1.second);
    show_result(res_1_ans);

    auto res_2_ans = s_ans.nextGreaterElement(case_2.first, case_2.second);
    show_result(res_2_ans);

    auto res_4_ans = s_ans.nextGreaterElement(case_4.first, case_4.second);
    show_result(res_4_ans);
}
