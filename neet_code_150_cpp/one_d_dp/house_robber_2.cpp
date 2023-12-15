#include <iostream>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
public:
    int rob(std::vector<int>& nums) {
        int n = nums.size();
        
        if (n == 1) {
            return nums[0];
        }
        
        int range1 = robber(nums, 0, n - 2);
        int range2 = robber(nums, 1, n - 1);
        
        return std::max(range1, range2);
    }
private:
    int robber(std::vector<int>& nums, int start, int end) {
        int prev = 0;
        int curr = 0;
        int next = 0;
        
        for (int i = start; i <= end; i++) {
            next = std::max(prev + nums[i], curr);
            prev = curr;
            curr = next;
        }
        
        return curr;
    }
};

// LeetCodeのACした提出例より(1msで一番速い)
class Solution {
    public:
        // こっちの方がdpっぽい
        int rob(std::vector<int>& nums) {
            int n = nums.size(); 

            if(n == 1) {
                return nums[0];
            } else if (n == 2) {
                return std::max(nums[0], nums[1]);
            }

            //dp1[i],dp2[i]:iまでのコスト 
            std::vector<int> dp1(n-1, 0), dp2(n-1, 0);

            // dp1:  num[0]からスタートするdpテーブル
            // for dp1
            dp1[0] = nums[0]; 
            dp1[1] = std::max(nums[0], nums[1]);

            for(int i = 2; i < n-1; i++) {
                dp1[i] = std::max(nums[i] + dp1[i-2], dp1[i-1]);
            }

            // dp2:  num[1]からスタートするdpテーブル
            // for dp2
            dp2[0] = nums[1]; 
            dp2[1] = std::max(nums[1], nums[2]);

            for(int i = 3; i < n; i++) {
                dp2[i-1] = std::max(nums[i] + dp2[i-3], dp2[i-2]);
            }

            return std::max(dp1[n-2], dp2[n-2]);
        }

        // AC
        // 上のrob()もとに自分で考えた
        int rob_2(std::vector<int>& nums) {
            int n = nums.size();

            if(n == 1) {
                return nums[0];
            } else if (n == 2) {
                return std::max(nums[0], nums[1]);
            }

            // dp_1[i], dp_2[i]: i - 1までのコスト
            std::vector<int> dp_1(n + 1, -(1<<30)), dp_2(n + 1, -(1<<30)); 

            //dp_1: nums[0]スタート
            dp_1[0] = 0;
            dp_1[1] = nums[0];
            //dp_2: nums[1]スタート
            dp_2[0] = 0;
            dp_2[1] = 0;
            dp_2[2] = nums[1];

            // nums[0]をスタートした場合はi = n - 1まで
            // nums[1]をスタートした場合はi = nまで
            for(int i = 2; i < n + 1; ++i) {
                if(i <= n - 1) {
                    dp_1[i] = std::max(dp_1[i - 1], dp_1[i - 2] + nums[i - 1]);
                }

                if(i > 2) {
                    dp_2[i] = std::max(dp_2[i - 1], dp_2[i - 2] + nums[i - 1]);
                }
            }


            return std::max(dp_1[n - 1], dp_2[n]);
        }
};

int main(void) {
    auto case_1 = std::vector{2, 3, 2}; //3
    auto case_2 = std::vector{1, 2, 3, 1}; //4
    auto case_3 = std::vector{1, 2, 3}; //3

    SolutionAns s_ans;

    std::cout << s_ans.rob(case_1) << std::endl;
    std::cout << s_ans.rob(case_2) << std::endl;
    std::cout << s_ans.rob(case_3) << std::endl;

    Solution s_1;
    std::cout << s_1.rob(case_1) << std::endl;
    std::cout << s_1.rob(case_2) << std::endl;
    std::cout << s_1.rob(case_3) << std::endl;

    std::cout << s_1.rob_2(case_1) << std::endl;
    std::cout << s_1.rob_2(case_2) << std::endl;
    std::cout << s_1.rob_2(case_3) << std::endl;
}
