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

// 初見で解けなかった
class Solution {
    public:
        void reorderList(ListNode* head) {
            auto current = head;
            auto tmp_head = head;
            while(current->next->next) {
#ifdef DEBUG_HOGE
                std::cout << "val " << current->val << std::endl;
#endif
                current = current->next;
            }
#ifdef DEBUG_HOGE
                std::cout << "val " << current->val << std::endl;
#endif
        }
};

// 模範解答
class SolutionAns {
    public:
        void reorderList(ListNode* head) {
            if (head->next == NULL) {
                return;
            }

            ListNode* prev = NULL;
            ListNode* slow = head;
            ListNode* fast = head;

            /*
             *  prev: slowの一個前
             *  slow: 一個づつListを進む
             *  fast: 二個づつListを進む
             *
             *   1 -> 2 -> 3 -> 4 -> nullptr
             *   が与えられたら
             *   prev = 2 ノード
             *   slow = 3 ノード
             *   fast = 4 ノード
             *   を指してた状態でbreakする
             */
            while (fast != NULL && fast->next != NULL) {
                prev = slow;
                slow = slow->next;
                fast = fast->next->next;
#ifdef DEBUG_HOGE
                    std::cout << "prev: " << prev->val;
                    std::cout << " slow: " << slow->val;
                    std::cout << std::endl;
#endif
            }

            prev->next = NULL;

            // 先頭にslow以降の部分Listをひっくり返したものをマージする
            ListNode* l1 = head;
            ListNode* l2 = reverse(slow);

            merge(l1, l2);
        }

     ListNode* test_reverse(ListNode* head) {
        return reverse(head);
    }

    void test_merge(ListNode* list_1, ListNode* list_2) {
        merge(list_1, list_2);
    }

    private:
        // 単にリストをひっくり返す
        /* 例:
         *  1 -> 2 -> 3 -> nullptr 
         *  =>
         *  3 -> 2 -> 1 -> nullptr
         */
        ListNode* reverse(ListNode* head) {
            ListNode* prev = NULL;
            ListNode* curr = head;
            ListNode* next = curr->next;

            while (curr != NULL) {
                next = curr->next;
                curr->next = prev;
                prev = curr;
                curr = next;
            }

            return prev;
        }

        // ２つのリストを一個ずらしてマージする
        /*例 list_1: 1 -> 2 -> nullptr
         *   list_2: 4 -> 3 -> nullptr
         *   =>
         *   list_1: 1 -> 4 -> 2 -> 3 -> nullptr
         */
        void merge(ListNode* l1, ListNode* l2) {
            while (l1 != NULL) {
                ListNode* p1 = l1->next;
                ListNode* p2 = l2->next;

                l1->next = l2;
                if (p1 == NULL) {
                    break;
                }
                l2->next = p1;

                l1 = p1;
                l2 = p2;
            }
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
    SolutionAns s_ans;
    //s_1.reorderList(&l_1_0);
    show_list(&l_1_0);

    s_ans.reorderList(&l_1_0);
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
    s_ans.reorderList(&l_2_0);
    //s_1.reorderList(&l_2_0);
    show_list(&l_2_0);


    // 1 -> 2 -> nullptr
    ListNode l_3_0(1);
    ListNode l_3_1(2);
    l_3_0.next = &l_3_1;

    show_list(&l_3_0);
    s_ans.reorderList(&l_3_0);
    show_list(&l_3_0);

    // 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> nullptr
    ListNode l_4_0(0);
    ListNode l_4_1(1);
    ListNode l_4_2(2);
    ListNode l_4_3(3);
    ListNode l_4_4(4);
    ListNode l_4_5(5);
    ListNode l_4_6(6);
    ListNode l_4_7(7);
    l_4_0.next = &l_4_1;
    l_4_1.next = &l_4_2;
    l_4_2.next = &l_4_3;
    l_4_3.next = &l_4_4;
    l_4_4.next = &l_4_5;
    l_4_5.next = &l_4_6;
    l_4_6.next = &l_4_7;

    show_list(&l_4_0);
    s_ans.reorderList(&l_4_0);
    show_list(&l_4_0);


    // reverse(), merge()の挙動
    std::cout << "reverse()の挙動" << std::endl;
    ListNode t_0_0(0);
    ListNode t_0_1(11);
    ListNode t_0_2(22);
    t_0_0.next = &t_0_1;
    t_0_1.next = &t_0_2;
    show_list(&t_0_0);
    show_list(s_ans.test_reverse(&t_0_0));

    std::cout << "merge()の挙動" << std::endl;
    ListNode t_1_0(11);
    ListNode t_1_1(22);
    ListNode t_1_2(33);
    t_1_0.next = &t_1_1;
    t_1_1.next = &t_1_2;

    ListNode t_2_0(444);
    ListNode t_2_1(333);
    ListNode t_2_2(222);
    t_2_0.next = &t_2_1;
    t_2_1.next = &t_2_2;

    show_list(&t_1_0);
    show_list(&t_2_0);
    s_ans.test_merge(&t_1_0, &t_2_0);
    show_list(&t_1_0);
}
