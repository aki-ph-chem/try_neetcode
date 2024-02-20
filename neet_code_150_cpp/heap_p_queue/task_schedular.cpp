#include <iostream>
#include <vector>
#include <queue>
#define DEBUG

// 模範解答 1
class SolutionAns1 {
    public:
        int leastInterval(std::vector<char>& tasks, int n) {
            std::vector<int> counter(26);

            int maxCount = 0;
            int maxCountFrequency = 0;

            for(int i = 0; i < tasks.size(); ++i) {
                ++counter[tasks[i] - 'A'];
                int currentCount = counter[tasks[i] - 'A'];

                if(maxCount == currentCount) {
                    ++maxCountFrequency;
                } else if(maxCount < currentCount) {
                    maxCount = currentCount;
                    maxCountFrequency = 1;
                }
            }

            int partCount = maxCount - 1;
            int partLenght = n - (maxCountFrequency - 1);
            int emptySlots = partCount * partLenght;
            int availableTasks = tasks.size() - maxCount * maxCountFrequency;
            int idles = std::max(0, emptySlots - availableTasks);

            return tasks.size() + idles;
        }
};

// 模範解答2
class SolutionAns2 {
    public:
        int leastInterval(std::vector<char>& tasks, int n) {
            std::priority_queue<int> pq;
            std::queue<std::vector<int>> q;
            std::vector<int> counter(26);

            for(auto& task: tasks) {
                ++counter[task - 'A'];
            }
            for(int i = 0; i < 26; ++i) {
                if(counter[i]) {
                    pq.push(counter[i]);
                }
            }

#ifdef DEBUG
            std::cout << "### pq ###" << std::endl;
            auto pq_copy = pq;
            while(!pq_copy.empty()) {
                auto x = pq_copy.top();
                std::cout << x << " ";
                pq_copy.pop();
            }
            std::cout << std::endl;
            std::cout << "### pq ###" << std::endl;

            std::cout << "### q ###" << std::endl;
            auto q_copy = q;
            while(!q_copy.empty()) {
                auto x = q_copy.front();
                if(x.size() > 2) {
                    std::cout << x[0] << x[1] << '\n';
                }
            }
            std::cout << std::endl;
            std::cout << "### q ###" << std::endl;
#endif

            int time = 0;
            while(!q.empty() || !pq.empty()) {
                ++time;
                if(!pq.empty()) {
                    if(pq.top() - 1) {
                        q.push({pq.top() - 1, time + n});
                    }
                        pq.pop();
                }

                if(!q.empty() && q.front()[1] == time) {
                    pq.push(q.front()[0]);
                    q.pop();
                }
            }

            return time;
        }
};

int main(void) {
    using Case = std::pair<std::vector<char>,int>;

    Case case_1 = {{'A', 'A', 'A', 'B', 'B', 'B'}, 2};
    Case case_2 = {{'A', 'A', 'A', 'B', 'B', 'B'}, 0};

    SolutionAns1 s_ans_1;
    std::cout << s_ans_1.leastInterval(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_1.leastInterval(case_2.first, case_2.second) << std::endl;

    SolutionAns2 s_ans_2;
    std::cout << s_ans_2.leastInterval(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_2.leastInterval(case_2.first, case_2.second) << std::endl;
}
