#include <iostream>
#include <vector>
#include <string>

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

void show_state(const ListNode* const node, std::string&& msg) {
    if(!node) {
        std::cout << "nullptr" << std::endl;
        return;
    }

    std::cout << msg << std::endl;
    std::cout <<"val: " << node->val 
        << " val next: " << node->next->val << std::endl;
}

// 参考
// ２つのソート済みの配列をマージする
std::vector<int> merge_array(const std::vector<int>& arr1, const std::vector<int>& arr2) {
    std::vector<int> result(arr1.size() + arr2.size(), 0);
    std::size_t i_1 = 0, i_2 = 0;
    for(std::size_t i = 0; i < result.size(); ++i) {
        if(arr1[i_1] <= arr2[i_2] && i_1 < arr1.size()) {
            result[i] = arr1[i_1];
            ++i_1;
        } else {
            result[i] = arr2[i_2];
            ++i_2;
        }
    }

    return result;
}

// 初見で解けなかった
// newしないで解くのは無理っぽい
class Solution {
    public:
        ListNode* mergeTwoList(ListNode* list1, ListNode* list2) {
            // NULLチェック
            if(!list1 && !list2) {
                return nullptr;
            }
            if(!list1){
                return list2;
            }
            if(!list2){
                return list1;
            }

            auto current = new ListNode();
            // 返す用に頭のポインタを保持
            auto result = current;
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

            // 最後の要素
            if(!list1) {
                current->next = list2;
            } else {
                current->next = list1;
            }

            // 最初に0が入っていいる要素をスキップ
            return result->next;
        }
};

// 模範解答
class SolutionAns {
    public:
        ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
            auto list_1 = list1, list_2 = list2;
            // NULLチェック
            if (list_1 == NULL && list_2 == NULL) {
                return NULL;
            }
            if (list_1 == NULL) {
                return list_2;
            }
            if (list_2 == NULL) {
                return list_1;
            }

            ListNode* dummy = new ListNode();
            ListNode *curr = dummy;
            while (list_1 != NULL && list_2 != NULL) {
                if (list_1->val <= list_2->val) {
                    curr->next = list_1;
                    list_1 = list_1->next;
                } else {
                    curr->next = list_2;
                    list_2 = list_2->next;
                }
                curr = curr->next;
            }

            // 最後の要素
            if (list_1 == NULL) {
                curr->next = list_2;
            } else {
                curr->next = list_1;
            }

            return dummy->next;
        }
};


int main(void) {
    ListNode l_1_0(1);
    ListNode l_1_1(2);
    ListNode l_1_2(4);
    l_1_0.next = &l_1_1;
    l_1_1.next = &l_1_2;
    //show_list(&l_1_0);

    ListNode l_2_0(1);
    ListNode l_2_1(3);
    ListNode l_2_2(4);
    l_2_0.next = &l_2_1;
    l_2_1.next = &l_2_2;
    //show_list(&l_2_0);

    Solution s_1;
    show_list(s_1.mergeTwoList(&l_1_0, &l_2_0));


    SolutionAns s_2;
    //auto res_2 = s_2.mergeTwoLists(&l_1_0, &l_2_0);
    //show_list(res_2);
    
    /*
    auto a_1 = std::vector{1,2,3};
    auto a_2 = std::vector{4,5,6};
    auto res_a = merge_array(a_1, a_2);
    for(const auto& v: res_a) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    auto b_1 = std::vector{1,2,3};
    auto b_2 = std::vector{1,2,3};
    auto res_b = merge_array(b_1, b_2);
    for(const auto& v: res_b) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    auto c_1 = std::vector{1,3,5};
    auto c_2 = std::vector{2,4,6};
    auto res_c = merge_array(c_1, c_2);
    for(const auto& v: res_c) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    */
}

