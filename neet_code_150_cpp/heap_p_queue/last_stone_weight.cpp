#include <iostream>
#include <vector>
#include <queue>

// AC
class Solution {
    public:
        int lastStoneWeight(std::vector<int>& stones) {
            std::priority_queue<int> max_heap(stones.begin(), stones.end());

            while(max_heap.size() > 1) {
                auto y = max_heap.top();
                max_heap.pop();
                auto x = max_heap.top();
                max_heap.pop();

                if(x != y) {
                    max_heap.push(y - x);
                }
            }

            if(max_heap.size() == 0) {
                return 0;
            }

            return max_heap.top();
        }
};

// 模範解答
class SolutionAns {
    public:
        int lastStoneWeight(std::vector<int>& stones) {
            std::priority_queue<int> pq;
            for(auto& v: stones) {
                pq.push(v);
            }

            while(pq.size() > 1) {
                int y = pq.top();
                pq.pop();
                int x = pq.top();
                pq.pop();

                if(y > x) {
                    pq.push(y - x);
                }
            }

            if(pq.empty()) {
                return 0;
            }

            return pq.top();
        }
};

int main(void) {
    auto case_1 = std::vector{2, 7, 4, 1, 8, 1};
    auto case_2 = std::vector{1};

    Solution s_1;
    std::cout << s_1.lastStoneWeight(case_1) << std::endl;
    std::cout << s_1.lastStoneWeight(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.lastStoneWeight(case_1) << std::endl;
    std::cout << s_ans.lastStoneWeight(case_2) << std::endl;
}
