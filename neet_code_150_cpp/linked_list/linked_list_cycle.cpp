#include <iostream>
#include <unordered_set>

//#define DEBUG
//#define CXX_20

// singly-linked list
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x)
        :val(x), next(nullptr){}
};

// AC
// 方針: ダブり判定 -> setを使う
class Solution {
    public:
        bool hasCycle(ListNode* head) {
            // NULLチェック
            if(!head) {
                return false;
            }
            std::unordered_set<ListNode*> set;

            auto node_current = head;
            while(node_current->next) {

#ifdef DEBUG
                std::cout << node_current->val << " ";
#endif

#ifdef CXX_20
                if(set.contains(node_current)) {
#else
                if(set.find(node_current) != set.end()) {
#endif
                    return true;
                } 

                set.insert(node_current);
                node_current = node_current->next;
            }

#ifdef DEBUG
            std::cout << std::endl;
#endif

            return false;
        }
};

// 模範解答
class SolutionAns {
    public:
        // ?
        bool hasCycle(ListNode* head) {
            if (head == NULL) {
                return false;
            }

            ListNode* slow = head;
            ListNode* fast = head;

            // 一個ずつNodeをたどるものと2個ずつNodeをたどるものを考える
            while (fast->next != NULL && fast->next->next != NULL) {
                slow = slow->next;
                fast = fast->next->next;
                if (slow == fast) {
                    return true;
                }
            }

            return false;
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
    // true

    // 1 -> 2
    // ^    |
    // | <- v
    ListNode l_2_0(1);
    ListNode l_2_1(2);
    l_2_0.next = &l_2_1;
    l_2_1.next = &l_2_0;
    // true

    // 1 -> nullptr
    ListNode l_3_0(1);
    l_3_0.next = nullptr;
    // false

    // 1 -> 2 -> nullptr
    ListNode l_4_0(1);
    ListNode l_4_1(2);
    l_4_0.next = &l_4_1;
    l_4_1.next = nullptr;
    // false
    
    Solution s_1;
    std::cout << s_1.hasCycle(&l_1_0) << std::endl;
    std::cout << s_1.hasCycle(&l_2_0) << std::endl;
    std::cout << s_1.hasCycle(&l_3_0) << std::endl;
    std::cout << s_1.hasCycle(&l_4_0) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.hasCycle(&l_1_0) << std::endl;
    std::cout << s_ans.hasCycle(&l_2_0) << std::endl;
    std::cout << s_ans.hasCycle(&l_3_0) << std::endl;
    std::cout << s_ans.hasCycle(&l_4_0) << std::endl;
}
