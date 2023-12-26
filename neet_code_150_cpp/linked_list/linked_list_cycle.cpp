#include <iostream>

// singly-linked list
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x)
        :val(x), next(nullptr){}
};

class Solution {
    public:
        bool hasCycle(ListNode* head) {

            return true;
        }
};

int main(void){
    // 3 -> 2 -> 0 -> -4
    //      ^          |
    //      |          v
    //       - < - < - 
    ListNode l_1_0(3);
    ListNode l_1_1(2);
    ListNode l_1_2(0);
    ListNode l_1_3(-4);
    l_1_0.next = &l_1_1;
    l_1_1.next = &l_1_2;
    l_1_2.next = &l_1_3;
    l_1_3.next = &l_1_1;

    // 1 -> 2
    // ^    |
    // | <- v
    ListNode l_2_0(1);
    ListNode l_2_1(2);
    l_2_0.next = &l_2_1;
    l_2_1.next = &l_2_0;

    // 1
    ListNode l_3_0(1);
    l_3_0.next = nullptr;

    Solution s_1;
}
