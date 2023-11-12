#include <cstdint>
#include <iostream>

#define DEBUG_HOGE

//Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// リストを表示
void show_list(const ListNode* const node) {
    auto node_current = node;
    while(node_current->next) {
        std::cout << node_current->val << "->"; 
        node_current = node_current->next;
    }
    std::cout << node_current->val << std::endl;
}

class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        auto node_prev = head;
        auto node_current = head->next;
        auto node_tmp = node_current;

        node_prev->next = nullptr;
        node_current->next = node_prev;

        node_tmp = node_current->next;
    }
};
int main(void) {
    // 1 -> 2 -> 3 -> 4 -> 5
    ListNode l_1_0(1);
    ListNode l_1_1(2);
    ListNode l_1_2(3);
    ListNode l_1_3(4);
    ListNode l_1_4(5);
    l_1_0.next = &l_1_1;
    l_1_1.next = &l_1_2;
    l_1_2.next = &l_1_3;
    l_1_3.next = &l_1_4;
    show_list(&l_1_0);

    Solution s_1;
    auto rev_l_1_0 = s_1.reverseList(&l_1_0);
    show_list(rev_l_1_0);
}
