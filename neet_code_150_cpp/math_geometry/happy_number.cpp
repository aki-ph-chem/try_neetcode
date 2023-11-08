#include <iostream>
#include <vector>
#include <cmath>
#include <unordered_set>

//AC
class Solution {
    public:
        bool is_happy(int n) {
            auto m = n;
            std::unordered_set<int> set;

            while(m != 1) {
                std::vector<int> digits;
                while(m > 0) {
                    digits.push_back(m % 10);
                    m /= 10;
                }
                m = sq_sum(digits);
                if(set.find(m) != set.end()) {
                    return false;
                } else {
                    set.insert(m);
                }
            }

            return true;
        }

    private:
        int sq_sum(const std::vector<int>& vec) {
            int sq_sum = 0;
            for(const auto& x: vec) {
                sq_sum += x * x;
            }
            return sq_sum;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isHappy(int n) {
            int slow = n;
            int fast = getNext(n);

            while (slow != fast && fast != 1) {
                slow = getNext(slow);
                fast = getNext(getNext(fast));
            }

            if (fast == 1) {
                return true;
            }
            return false;
        }
    private:
        int getNext(int n) {
            int sum = 0;
            while (n > 0) {
                int digit = n % 10;
                n /= 10;
                sum += std::pow(digit, 2);
            }
            return sum;
        }
};

int main(void) {
    Solution s_1;
    std::cout << "case_1: " << s_1.is_happy(19) << std::endl;
    std::cout << "case_2: " << s_1.is_happy(2) << std::endl;
    std::cout << "case_3: " << s_1.is_happy(12) << std::endl;

    SolutionAns s_2;
    std::cout << "case_1: " << s_2.isHappy(19) << std::endl;
    std::cout << "case_2: " << s_2.isHappy(2) << std::endl;
    std::cout << "case_3: " << s_2.isHappy(12) << std::endl;
}
