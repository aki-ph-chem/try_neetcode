#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_map>

// def of Interval
class Interval {
    public:
        int start,end;

        Interval(int start, int end) {
            this->start = start;
            this->end = end;
        }
};

class Solution {
    public:
        int minMeetingRooms(std::vector<Interval>& intervals) {
            int n = intervals.size();
            if(n == 0) {
                return 0;
            }

            std::sort(intervals.begin(), intervals.end(),
                    [](auto& a, auto& b) {return a.start < b.start;});

            int result = 1;
            auto interval_prev = intervals[0];
            for(int i = 1; i < n; ++i) {
                if(intervals[i].start < interval_prev.end) {
                    ++result;
                } else {
                    interval_prev = intervals[i];
                }
            }

            return result;
        }

        int minMeetingRoomsB(std::vector<Interval>& intervals) {
            int n = intervals.size();
            if(n == 0) {
                return 0;
            }

            std::sort(intervals.begin(), intervals.end(),
                    [](auto& a, auto& b) {return a.start < b.start;});

            int result = 1;
            for(int i = 1; i < n; ++i) {
                if(intervals[i].start < intervals[i - 1].end) {
                    if(intervals[i - 1].end < intervals[i].end) {
                        intervals[i] = intervals[i - 1];
                    }
                    ++result;
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int minMeetingRooms(std::vector<Interval>& intervals) {
            std::vector<int> starts, ends;
            for (const auto& i : intervals) {
                starts.emplace_back(i.start);
                ends.emplace_back(i.end);
            }

            sort(starts.begin(), starts.end());
            sort(ends.begin(), ends.end());

            int min_rooms = 0, cnt_rooms = 0;
            int s = 0, e = 0;
            while (s < starts.size()) {
                if (starts[s] < ends[e]) {
                    ++cnt_rooms;  

                    min_rooms = std::max(min_rooms, cnt_rooms);
                    ++s;
                } else {
                    --cnt_rooms;  
                    ++e;
                }
            }
            return min_rooms;
        }
};

int main(void) {
    auto case_1 = std::vector{Interval{0,30}, Interval{5, 10}, Interval{15, 20}};
    // => 2 
    auto case_2 = std::vector{Interval{4, 9}};
    // => 1
    auto case_3 = std::vector{Interval{1, 5}, Interval{2, 6}, Interval{3, 7}, Interval{4, 8}, Interval{5, 9}};
    // => 4

    /*
    Solution s_1;
    std::cout << s_1.minMeetingRooms(case_1) << std::endl;
    std::cout << s_1.minMeetingRooms(case_2) << std::endl;
    std::cout << s_1.minMeetingRooms(case_3) << std::endl;

    std::cout << s_1.minMeetingRoomsB(case_1) << std::endl;
    std::cout << s_1.minMeetingRoomsB(case_2) << std::endl;
    std::cout << s_1.minMeetingRoomsB(case_3) << std::endl;
    */

    SolutionAns s_ans;
    std::cout << s_ans.minMeetingRooms(case_1) << std::endl;
    std::cout << s_ans.minMeetingRooms(case_2) << std::endl;
    std::cout << s_ans.minMeetingRooms(case_3) << std::endl;
}
