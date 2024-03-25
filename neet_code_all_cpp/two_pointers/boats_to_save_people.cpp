#include <complex>
#include <iostream>
#include <pthread.h>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        int numRescueBoats(std::vector<int>& people, int limit) {
            std::sort(people.begin(), people.end());

            int boatRequired = 0;
            int lightestPerson = 0;
            int heaviesPerson = people.size() - 1;

            while(lightestPerson <= heaviesPerson) {
                if(people[lightestPerson] + people[heaviesPerson] <= limit) {
                    --heaviesPerson;
                    ++lightestPerson;
                } else {
                    --heaviesPerson;
                }
                ++boatRequired;
            }

            return boatRequired;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 2}, 3};
    // 1: {{1,2})
    Case case_2 = {{3, 2, 2, 1}, 3};
    // 3: {{1,2}, {2), {3))
    Case case_3 = {{3, 5, 3, 4}, 5};
    // 4: {{3},{3),{4),{5))
    Case case_4 = {{3, 2, 3, 2, 2}, 6};
    // 3:

    SolutionAns s_ans;

    std::cout << s_ans.numRescueBoats(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.numRescueBoats(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.numRescueBoats(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.numRescueBoats(case_4.first, case_4.second) << std::endl;
}
