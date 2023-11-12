#include <algorithm>
#include <iostream>
#include <vector>
#include <unordered_map>

//#define DEBUG

// 模範解答
class DetectSquaresAns { 
    public:
        DetectSquaresAns() {}

        void add(std::vector<int> point) {
            points[point[0]][point[1]]++;
        }

        int count(std::vector<int> point) {
            int x1 = point[0];
            int y1 = point[1];

            int result = 0;

            for (auto x = points.begin(); x != points.end(); x++) {
                std::unordered_map<int, int> yPoints = x->second;
                for (auto y = yPoints.begin(); y != yPoints.end(); y++) {
                    int x3 = x->first;
                    int y3 = y->first;

                    // skip points on same x-axis or y-axis
                    if (abs(x3 - x1) == 0 || abs(x3 - x1) != abs(y3 - y1)) {
                        continue;
                    }

                    result += points[x3][y3] * points[x1][y3] * points[x3][y1];
                }
            }

            return result;
        }
    private:
        // {x -> {y -> count}}
        std::unordered_map<int, std::unordered_map<int, int>> points;
};

// Rustの模範解答より
// 答えが合わない
class DetectSquares {
    private:
        std::unordered_map<int, std::unordered_map<int, int>> counts;

    public:
        DetectSquares() {}

        void add(std::vector<int> point) {
            ++counts[point[0]][point[1]];
        }

        int count(std::vector<int> point) {
            int res = 0;
            int px = point[0], py = point[1];

            for(auto& [x, pair]: counts) {
                for(auto& [y, c]: pair) {
#ifdef DEBUG
                    std::cout << x << "," << y << std::endl;
#endif
                    if(std::abs(py - y) != std::abs(px - x) || x == px || y == py ) {
                        continue;
                    }

#ifdef DEBUG
                    std::cout << counts[x][py] << ", ";
                    std::cout << counts[px][y] << std::endl;
#endif

                    res += counts[x][py] * counts[px][y]; 
                }

            }
            return res;
        }

#ifdef DEBUG
        void show(void) {

            for(const auto& [x, y_num]: counts) {
                for(const auto& [y, num]: y_num) {
                    std::cout << x << "," << y <<": " << num << std::endl;
                }
            }
        }
#endif
};

int main(void) {
    DetectSquares p_1;
    p_1.add(std::vector{3, 10});
    p_1.add(std::vector{11, 2});
    p_1.add(std::vector{3, 2});
#ifdef DEBUG
    p_1.show();
#endif
    std::cout << p_1.count(std::vector{11, 10}) << std::endl; 
    std::cout << p_1.count(std::vector{14, 8}) << std::endl; 
    p_1.add(std::vector{11, 2});
    std::cout << p_1.count(std::vector{11, 10}) << std::endl; 

    DetectSquaresAns p_2;
    p_2.add(std::vector{3, 10});
    p_2.add(std::vector{11, 2});
    p_2.add(std::vector{3, 2});
    std::cout << p_2.count(std::vector{11, 10}) << std::endl; 
    std::cout << p_2.count(std::vector{14, 8}) << std::endl; 
    p_2.add(std::vector{11, 2});
    std::cout << p_2.count(std::vector{11, 10}) << std::endl; 
}
