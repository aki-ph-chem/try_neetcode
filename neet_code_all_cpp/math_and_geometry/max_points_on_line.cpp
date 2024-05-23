#include <iostream>
#include <climits>
#include <vector>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        // AC
        // 浮動小数点型を辞書のkeyにしているのがちょっと不安
        int maxPoints(std::vector<std::vector<int>>& points) {
            auto slope = [](std::vector<int>& p1, std::vector<int>& p2) {
                if(p2[0] == p1[0]) {
                    return (float)INT_MAX;
                }
                return (float)(p2[1] - p1[1]) / (float)(p2[0] - p1[0]);
            };

            int result = 1;
            for(int i = 0; i < points.size(); ++i) {
                std::unordered_map<float, int> count;
                for(int j = i + 1; j < points.size(); ++j) {
                    float s = slope(points[i], points[j]);
                    ++count[s];
                    result = std::max(result, count[s] + 1);
                }
            }

            return result;
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {{1, 1}, {2, 2}, {3, 3}};
    // => 3
    std::vector<std::vector<int>> case_2 = {{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}};
    // => 4
}
