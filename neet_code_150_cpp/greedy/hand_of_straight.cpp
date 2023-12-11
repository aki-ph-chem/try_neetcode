#include <iostream>
#include <vector>
#include <map>

// 模範解答
class SolutionAns {
public:
    bool isNStraightHand(std::vector<int>& hand, int groupSize) {
        int n = hand.size();
        
        if (n % groupSize != 0) {
            return false;
        }
        
        // map {card value -> count}
        std::map<int, int> m;
        for (int i = 0; i < n; i++) {
            m[hand[i]]++;
        }
        
        while (!m.empty()) {
            int curr = m.begin()->first;
            for (int i = 0; i < groupSize; i++) {
                // curr + 0, curr + 1, ..が存在しないならfalse
                if (m[curr + i] == 0) {
                    return false;
                }

                // 使用した数を削除
                m[curr + i]--;
                // 0個ならばmapから削除
                if (m[curr + i] < 1) {
                    m.erase(curr + i);
                }
            }
        }
        
        return true;
    }
};

int main(void) {
    auto case_1_vec = std::vector{1, 2, 3, 6, 2, 3, 4, 7, 8};
    auto case_1_num = 3;

    auto case_2_vec = std::vector{1, 2, 3, 4, 5};
    auto case_2_num = 4;

    SolutionAns s_ans;

    std::cout << s_ans.isNStraightHand(case_1_vec, case_1_num) << std::endl;
    std::cout << s_ans.isNStraightHand(case_2_vec, case_2_num) << std::endl;
}
