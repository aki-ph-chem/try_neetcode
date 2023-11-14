#include <iostream>

#define DEBUG_HOGE

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

class Solution {
    public:
        ListNode* mergeTwoList(ListNode* list1, ListNode* list2) {
            ListNode* result = list1;

            while(result->next && list2->next) {
                if(result->val >= list2->val) {
                }

                result = result->next;
                list2 = list2->next;
            }

            return result;
        }
};

int main(void) {
    ListNode l_1_0(1);
    ListNode l_1_1(2);
    ListNode l_1_2(4);
    l_1_0.next = &l_1_1;
    l_1_1.next = &l_1_2;
    show_list(&l_1_0);

    ListNode l_2_0(1);
    ListNode l_2_1(3);
    ListNode l_2_2(4);
    l_2_0.next = &l_2_1;
    l_2_1.next = &l_2_2;
    show_list(&l_2_0);
}

