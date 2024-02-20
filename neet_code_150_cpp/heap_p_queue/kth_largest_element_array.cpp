#include <exception>
#include <functional>
#include <iostream>
#include <vector>
#include <queue>

// AC
class Solution {
    public:
        int findKthLargest(std::vector<int>& nums, int k) {
            std::priority_queue<int> pq(nums.begin(), nums.end());

            for(int step = 0; step < k - 1; ++step) {
                pq.pop();
            }

            return pq.top();
        }
};

// Rustの模範解答
class SolutionAnsRust {
    public:
        // AC
        int findKthLargest(std::vector<int>& nums, int k) {
            // 降順のpriority_queue<int>を使う
            std::priority_queue<int> pq;

            for(auto& n: nums) {
                if((int)pq.size() < k) {
                    pq.push(-n);
                    continue;
                } else if(-pq.top() < n) {
                    pq.pop();
                    pq.push(-n);
                }
            }

            return -pq.top();
        }

        // AC
        int findKthLargestAscend(std::vector<int>& nums, int k) {
            // 昇順のpriority_queue<int>を使う
            std::priority_queue<int,
                std::vector<int>,
                std::greater<int>> pq;

            for(auto& n: nums) {
                if((int)pq.size() < k) {
                    pq.push(n);
                    continue;
                } else if(pq.top()< n) {
                    pq.pop();
                    pq.push(n);
                }
            }

            return pq.top();
        }
};

// C++の模範解答
class SolutionAns {
    public:
        int findKthLargest(std::vector<int>& nums, int k) {
            std::priority_queue<int, 
                std::vector<int>, 
                std::greater<int>> pq;

            for(auto& n: nums) {
                pq.push(n);
                if (pq.size() > k) {
                    pq.pop();
                }
            }

            return pq.top();
        } 
};

int main(void){
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{3, 2, 1, 5, 6, 4}, 2};
    // => 5
    Case case_2 = {{3, 2, 3, 1, 2, 4, 5, 5, 6}, 4};
    // => 4

    Solution s_1;
    std::cout << s_1.findKthLargest(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.findKthLargest(case_2.first, case_2.second) << std::endl;

    SolutionAnsRust s_rs;
    std::cout << s_rs.findKthLargest(case_1.first, case_1.second) << std::endl;
    std::cout << s_rs.findKthLargest(case_2.first, case_2.second) << std::endl;
}
