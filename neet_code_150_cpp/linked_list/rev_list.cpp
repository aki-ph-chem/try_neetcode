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
    if(!node) {
        return;
    }

    auto node_current = node;
    while(node_current->next) {
        std::cout << node_current->val << "->"; 
        node_current = node_current->next;
    }
    std::cout << node_current->val << std::endl;
}

//AC
class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        if(!head) {
            return nullptr;
        } else if (!head->next) {
            return head;
        }
        auto node_p = head;
        auto node_c = head->next;

        node_p->next = nullptr;
        while(node_c->next) {
            auto node_tmp = node_c->next;
            node_c->next = node_p;
            node_p = node_c;
            node_c = node_tmp;
#ifdef DEBUG_HOGE
            if(node_p && node_c ){
                std::cout << "val_p " << node_p->val; 
                std::cout << "val_c " << node_c->val << std::endl; 
            }
#endif
        }
        node_c->next = node_p;

        return node_c;
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
    show_list(s_1.reverseList(&l_1_0));

    // 11 -> 22 
    ListNode l_2_0(11);
    ListNode l_2_1(22);
    l_2_0.next = &l_2_1;
    show_list(&l_2_0);
    show_list(s_1.reverseList(&l_2_0));

    // 111
    ListNode l_3_0(111);
    show_list(&l_3_0);
    show_list(s_1.reverseList(&l_3_0));

    // nullptr
    ListNode* l_4_0 = nullptr;
    show_list(l_4_0);
    show_list(s_1.reverseList(l_4_0));
}
