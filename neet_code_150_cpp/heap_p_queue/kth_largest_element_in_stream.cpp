#include <functional>
#include <iostream>
#include <vector>
#include <queue>

// AC
// Rustの模範解答より
class SolutionAnsRust {
    public:
        SolutionAnsRust(int k, std::vector<int>& nums)
            :size(k)
        {
            for(auto& n: nums) {
                this->add(n);
            }
        }

        int add(int val) {
            min_heap.push(val);

            if(min_heap.size() > size) {
                min_heap.pop();
            }

            return min_heap.top();
        }

    private:
        std::priority_queue<int,
            std::vector<int>,
            std::greater<int>> min_heap;
        std::size_t size;
};

// 模範解答
class SolutionAns {
    public:
        SolutionAns(int k, std::vector<int>& nums) {
            this->k = k;
            for(auto& v: nums) {
                pq.push(v);
            }

            while((int)pq.size() > this->k) {
                pq.pop();
            }
        }

        int add(int val){
            pq.push(val);
            if((int)pq.size() > k) {
                pq.pop();
            }

            return pq.top();
        }

    private:
        int k;
        std::priority_queue<int, 
            std::vector<int>,
            std::greater<int>> pq;
};

int main(void) {
    using Case = std::pair<int, std::vector<int>>;
    Case case_1 = {3, {4, 5, 8, 2}};
    auto case_1_query = std::vector{3, 5, 10, 9, 4};

    SolutionAnsRust s_1_ans_rs(case_1.first, case_1.second);
    for(auto& v: case_1_query) {
        std::cout << s_1_ans_rs.add(v) << '\n';
    }

    SolutionAns s_1_ans(case_1.first, case_1.second);
    for(auto& v: case_1_query) {
        std::cout << s_1_ans.add(v) << '\n';
    }
}
