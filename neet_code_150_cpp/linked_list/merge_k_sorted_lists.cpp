#include <iostream>
#include <vector>
#include <algorithm>

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

// 解けなかった
class Solution {
    public:
        ListNode* mergekLists(std::vector<ListNode*>& lists) {
            auto head = new ListNode(0);
            auto result = head->next; 

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        ListNode* mergekLists(std::vector<ListNode*>& lists) {
            auto n = (int)lists.size();
            if(!n) {
                return nullptr;
            }

            while(n > 1) {
                for(int i = 0; i < n / 2; ++i) {
                    lists[i] = mergeTwoLists(lists[i], lists[n - i - 1]);
                }
                n = (n + 1) / 2;
            }

            return lists.front();
        }

    private:
        ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
            if(!list1 && !list2) {
                return nullptr;
            }
            if(!list1) {
                return list2;
            }
            if(!list2) {
                return list1;
            }

            ListNode* head = nullptr;
            if(list1->val <= list2->val) {
                head = list1;
                list1 = list1->next;
            } else {
                head = list2;
                list2 = list2->next;
            }
            ListNode* current = head;

            while(list1 && list2) {
                if(list1->val <= list2->val) {
                    current->next = list1;
                    list1 = list1->next;
                } else {
                    current->next = list2;
                    list2 = list2->next;
                }
                current = current->next;
            }

            if(!list1) {
                current->next = list2;
            } else {
                current->next = list1;
            }

            return head;
        }
};

int main(void) {
    ListNode root_1(1);
    ListNode root_1_1(4);
    ListNode root_1_2(5);
    root_1.next = &root_1_1;
    root_1_1.next = &root_1_2;
    //show_list(&root_1);

    ListNode root_2(1);
    ListNode root_2_1(3);
    ListNode root_2_2(4);
    root_2.next = &root_2_1;
    root_2_1.next = &root_2_2;
    //show_list(&root_2);

    ListNode root_3(2);
    ListNode root_3_1(6);
    root_3.next = &root_3_1;
    //show_list(&root_3);

    std::vector<ListNode*> case_1;
    case_1.push_back(&root_1);
    case_1.push_back(&root_2);
    case_1.push_back(&root_3);

    SolutionAns s_ans;
    auto res_1 = s_ans.mergekLists(case_1);
    show_list(res_1);
}
