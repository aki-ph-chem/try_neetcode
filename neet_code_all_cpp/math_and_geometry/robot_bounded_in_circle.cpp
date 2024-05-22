#include <iostream>
#include <string>

// hit 1: 命令を一通り実行したあとの座標を計算してみよう
// hit 2: 経路が閉じるのは最終状態の座標が(0, 0) になっているときもしくは移動方向が(0,
// 1)でないときである

class Solution {
    public:
        // AC
        bool isRobotBounded(std::string instructions) {
            int x =0, y = 0;
            int d_x = 0, d_y = 1;

            for(auto& c: instructions) {
                switch(c) {
                    case 'G':
                        {
                            x += d_x;
                            y += d_y;
                            break;
                        }
                    case 'L':
                        {
                            int tmp = d_x;
                            d_x = d_y;
                            d_y = -tmp;
                            break;
                        }
                    default:
                        {
                            int tmp = d_x;
                            d_x = -d_y;
                            d_y = tmp;
                            break;
                        }
                }
            }

            if((x == 0 && y == 0) || (d_x != 0 || d_y != 1)) {
                return true;
            }
            return false;
        }
};

int main(void) {
    std::string case_1 = "GGLLGG";
    // => true
    std::string case_2 = "GG";
    // => false
    std::string case_3 = "GL";
    // => true

    Solution s_1;

    std::cout << s_1.isRobotBounded(case_1) << std::endl;
    std::cout << s_1.isRobotBounded(case_2) << std::endl;
    std::cout << s_1.isRobotBounded(case_3) << std::endl;
}
