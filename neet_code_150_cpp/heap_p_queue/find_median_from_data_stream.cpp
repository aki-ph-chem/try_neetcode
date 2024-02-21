#include <iostream>
#include <vector>
#include <queue>

class SolutionAns{
    private:
        std::priority_queue<int> lower;
        std::priority_queue<int, std::vector<int>, std::greater<int>> higher;

    public:
        SolutionAns() {}

        void addNum(int num) {
            if (lower.empty()) {
                lower.push(num);
                return;
            }

            if (lower.size() > higher.size()) {
                if (lower.top() > num) {
                    higher.push(lower.top());
                    lower.pop();
                    lower.push(num);
                } else {
                    higher.push(num);
                }
            } else {
                if (num > higher.top()) {
                    lower.push(higher.top());
                    higher.pop();
                    higher.push(num);
                } else {
                    lower.push(num);
                }
            }
        }

        double findMedian(void) {
            double result = 0.0;

            if(lower.size() == higher.size()) {
                result = lower.top() + (higher.top() - lower.top()) / 2.0;
            } else {
                if(lower.size() > higher.size()) {
                    result = lower.top();
                } else {
                    result = higher.top();
                }
            }
            

            return result;
        }
};

int main(void) {
    SolutionAns median_ans;
    median_ans.addNum(1);
    median_ans.addNum(2);
    std::cout << median_ans.findMedian() << std::endl;
    // => 1.5
    median_ans.addNum(3);
    std::cout << median_ans.findMedian() << std::endl;
    // => 2.0
}
