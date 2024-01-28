#include <iostream>
#include <vector>
#include <algorithm>

// def of Interval
class Interval {
    public:
        int start,end;

        Interval(int start, int end) {
            this->start = start;
            this->end = end;
        }
};

// AC
class Solution {
    public:
        bool canAttendMeetings(std::vector<Interval>& intervals) {
            int n = intervals.size();
            if(n <= 1) {
                return true;
            }
            std::sort(intervals.begin(), intervals.end(), [](auto& a, auto& b) { return a.start < b.start;});

            for(int i = 1; i < n; ++i) {
                if(intervals[i].start < intervals[i - 1].end) {
                    return false;
                }
            }

            return true;
        }
};

// 模範解答
// 自分の解答と同じ
class SolutionAns {
public:
    bool canAttendMeetings(std::vector<Interval>& intervals) {
        sort(intervals.begin(), intervals.end(),
             [](const Interval& x, const Interval& y) { return x.start < y.start; });
        for (int i = 1; i < intervals.size(); ++i) {
            if (intervals[i].start < intervals[i - 1].end) {
                return false;
            }
        }
        return true;
    }
};
void show_intervals(const std::vector<Interval>& intervals) {
    for(const auto&v: intervals) {
        std::cout << v.start << "," << v.end;
    }
}

int main(void) {
    auto case_1 = std::vector{Interval{0,30}, Interval{5, 10}, Interval{15, 20}};
    // => false
    auto case_2 = std::vector{Interval{5, 8}, Interval{9, 15}};
    // => true

    Solution s_1;
    std::cout << s_1.canAttendMeetings(case_1) << std::endl;
    std::cout << s_1.canAttendMeetings(case_2) << std::endl;
}
