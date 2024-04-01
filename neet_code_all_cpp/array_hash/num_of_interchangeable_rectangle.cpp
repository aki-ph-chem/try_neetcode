#include <iostream>
#include <vector>
#include <map>

class Solution {
    public:
        // AC
        long long interchangeableRectangles(std::vector<std::vector<int>>& rectangles) {
            std::map<std::vector<int>, std::vector<int>> map;
            for(int i = 0; i < rectangles.size(); ++i) {
                auto gcd_rct = gcd(rectangles[i][0], rectangles[i][1]);
                auto rct_reduced = std::vector{rectangles[i][0] / gcd_rct, rectangles[i][1] / gcd_rct};

                if(map.find(rct_reduced) != map.end()) {
                    map[rct_reduced].push_back(i);
                } else {
                    map.insert({rct_reduced, std::vector{i}});
                }

            }

            long long result = 0;
            for(auto& [v, idx_list]: map) {
                auto n = (long long)idx_list.size();
                result += n * (n - 1) / 2;
            }

            return result;
        }

        // AC
        long long interchangeableRectangles2(std::vector<std::vector<int>>& rectangles) {

            std::map<std::pair<int, int>, long long> map;
            for(int i = 0; i < rectangles.size(); ++i) {
                auto gcd_rct = gcd(rectangles[i][0], rectangles[i][1]);
                auto rct_reduced = std::pair{rectangles[i][0] / gcd_rct, rectangles[i][1] / gcd_rct};

                ++map[rct_reduced];
            }

            long long result = 0;
            for(auto& [v, n]: map) {
                result += n * (n - 1) / 2;
            }

            return result;
        }

        int gcd(int a, int b) {
            if(b == 0) {
                return a;
            }

            return gcd(b, a % b);
        }
};

// 模範解答
class SolutionAns {
    public:
        long long interchangeableRectangles(std::vector<std::vector<int>>& rectangles) {
            std::map<long double, int> hash;
            long long result = 0;

            for(int i = 0; i < rectangles.size(); ++i) {
                auto ratio = (long double)(rectangles[i][0]) / (long double)(rectangles[i][1]);

                if(hash.find(ratio) != hash.end()) {
                    ++hash[ratio];
                } else {
                    hash[ratio] = 1;
                }

            }

            for(auto& [v, num]: hash) {
                result += (long long)(num) * (long long)(num - 1) / 2;
            }

            return result;
        }
};

int main(void) {
    using Case = std::vector<std::vector<int>>;

    Case case_1 = {{4,8},{3,6},{10,20},{15,30}}; 
    // => 6
    Case case_2 = {{4,5},{7,8}};
    // => 0

    Solution s_1;

    std::cout << s_1.interchangeableRectangles(case_1) << std::endl;
    std::cout << s_1.interchangeableRectangles(case_2) << std::endl;

    std::cout << s_1.interchangeableRectangles2(case_1) << std::endl;
    std::cout << s_1.interchangeableRectangles2(case_2) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.interchangeableRectangles(case_1) << std::endl;
    std::cout << s_ans.interchangeableRectangles(case_2) << std::endl;
}
