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

// 初見では解けなかった
class Solution {
    public:
        ListNode* removeNthFromEnd(ListNode* head, int n) {
            if(!head){
                return nullptr;
            } else if (!head->next) {
                return nullptr;
            } 

            auto result = head;
            auto prev = head;
            auto curr = head;
            auto fast = head;
            // fastをn - 1個先のノードへ進めておく
            for(int i = 0; i < n - 1; ++i) {
                fast = fast->next;
            }

            if (!fast) {
                return head->next;
            }

            // fastが最後尾に着くまでcurr,fastを進める
            while(fast->next) {
                prev = curr;
                curr = curr->next;
                fast = fast->next;
            }
#ifdef DEBUG_HOGE
            std::cout << " prev.val: " << prev->val;
            std::cout << " curr.val: " << curr->val;
            std::cout << " fast.val: " << fast->val << std::endl;
#endif
            prev->next = curr->next;

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        ListNode* removeNthFromEnd(ListNode* head, int n) {
            if (head->next == NULL) {
                return NULL;
            }

            ListNode* slow = head;
            ListNode* fast = head;

            //fastをn個先に進める
            while (n > 0) {
                fast = fast->next;
                n--;
            }

            if (fast == NULL) {
                return head->next;
            }

            while (fast->next != NULL) {
                slow = slow->next;
                fast = fast->next;
            }

            slow->next = slow->next->next;
            return head;
        }
};

int main(void) {
    ListNode c_1_0(1);
    ListNode c_1_1(2);
    ListNode c_1_2(3);
    ListNode c_1_3(4);
    c_1_0.next = &c_1_1;
    c_1_1.next = &c_1_2;
    c_1_2.next = &c_1_3;
    show_list(&c_1_0);

    Solution s_1;
    // OK 
    std::cout << "case_1" << std::endl;
    auto res_1 = s_1.removeNthFromEnd(&c_1_0, 2);
    show_list(res_1);

    ListNode c_2_0(10);
    // OK 
    std::cout << "case_2" << std::endl;
    auto res_2 = s_1.removeNthFromEnd(&c_2_0, 1);
    show_list(res_2);

    // NG 
    ListNode c_3_0(111);
    ListNode c_3_1(222);
    c_3_0.next = &c_3_1;
    std::cout << "case_3" << std::endl;
    auto res_3 = s_1.removeNthFromEnd(&c_3_0, 2);
    show_list(res_3);

    // 模範解答
    std::cout << "模範解答" << std::endl;
    SolutionAns s_ans;

    ListNode c_4_0(111);
    ListNode c_4_1(222);
    ListNode c_4_2(333);
    ListNode c_4_3(444);
    c_4_0.next = &c_4_1;
    c_4_1.next = &c_4_2;
    c_4_2.next = &c_4_3;
    std::cout << "case_4" <<std::endl;
    show_list(&c_4_0);
    auto res_4 = s_ans.removeNthFromEnd(&c_4_0, 2);
    show_list(res_4);

    ListNode c_5_0(1212);
    ListNode c_5_1(1313);
    c_5_0.next = &c_5_1;
    std::cout << "case_5" << std::endl;
    show_list(&c_5_0);
    auto res_5 = s_ans.removeNthFromEnd(&c_5_0, 2);
    show_list(res_5);
}
