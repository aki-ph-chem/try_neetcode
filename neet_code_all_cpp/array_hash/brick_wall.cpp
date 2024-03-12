#include <iostream>
#include <vector>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        int leastBricks(std::vector<std::vector<int>>& wall) {
            std::unordered_map<int,int> end_count;
            int max_end_count = 0;
            int rows = wall.size();

            for(int i = 0; i < rows; ++i) {
                int end_of_brick = 0; 
                int cols = wall[i].size() - 1;

                for(int j = 0; j < cols; ++j) {
                    end_of_brick += wall[i][j];

                    if(end_count.find(end_of_brick) != end_count.end()) {
                        ++end_count[end_of_brick];
                    } else {
                        end_count[end_of_brick] = 1;
                    }

                    max_end_count = std::max(max_end_count, end_count[end_of_brick]);
                }
            }

            return rows - max_end_count;
        }
};

int main(void) {
    using Case = std::vector<std::vector<int>>;

    Case case_1 = {{1,2,2,1},{3,1,2},{1,3,2},{2,4},{3,1,2},{1,3,1,1}};
    // => 2
    Case case_2 = {{1},{1},{1}};
    // => 3

    SolutionAns s_ans;
    std::cout<< s_ans.leastBricks(case_1) << std::endl;
    std::cout<< s_ans.leastBricks(case_2) << std::endl;
}
