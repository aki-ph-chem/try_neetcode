#include <iostream>
#include <vector>

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

class Solution {
    public:
        void reorderList(ListNode* head) {
            auto tmp_head = head;
            std::vector<ListNode*> vec_node;
            while(head) {
                vec_node.push_back(head);
                head = head->next;
            }
#ifdef DEBUG_HOGE
            for(const auto& v: vec_node) {
                std::cout <<"address: " << v ;
                std::cout <<" value: " << v->val << std::endl;
            }
#endif
            head = tmp_head;

            vec_node[3]->next = head->next;
            head->next = vec_node[3];
            head = head->next;

            /*
            vec_node[2]->next = head->next;
            head->next = vec_node[2];
            head = head->next;
            */

            vec_node[1]->next = head->next;
            head->next = vec_node[1];
            head = head->next;

            /*
            for(int i = static_cast<int>(vec_node.size() - 1); i > 0; --i) {
                vec_node[i]->next = head->next;
                head->next = vec_node[i];

                head = head->next;
            }
            */
            // 端点の処理
            head->next = nullptr;

            head = tmp_head;
        }
};

int main(void) {
    // 1 -> 2 -> 3 -> nullptr
    ListNode l_1_0(1);
    ListNode l_1_1(2);
    ListNode l_1_2(3);
    l_1_0.next = &l_1_1;
    l_1_1.next = &l_1_2;
    show_list(&l_1_0);

    Solution s_1;
    //s_1.reorderList(&l_1_0);
    show_list(&l_1_0);

    // 1 -> 2 -> 3 -> 4 -> nullptr
    ListNode l_2_0(1);
    ListNode l_2_1(2);
    ListNode l_2_2(3);
    ListNode l_2_3(4);
    l_2_0.next = &l_2_1;
    l_2_1.next = &l_2_2;
    l_2_2.next = &l_2_3;
    show_list(&l_2_0);
    s_1.reorderList(&l_2_0);
    show_list(&l_2_0);
}

