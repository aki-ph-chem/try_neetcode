## Merge Two Sorted List

２つのsortされた連結リストlist1, list2をマージして一つのsortされた連結リストをつくる

```mermaid
graph LR
list1 --- 1(1) --> 2(8) --> 3(9)
list2 --- 11(3) --> 12(5) --> 13(11)
```

まずlist1,list2のNULLチェックを行う。

```C++
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
```

続いて作業用の新しい連結リストcurrentを動的に生成して、最後に結果を出力するために先頭のポインタを別の変数resultに保持させておく。

```C++
// 作業用
auto current = new ListNode();
// 最後に結果を出力する用
auto result = current;
```

続いてリストをマージするためにlist1 -> val, list2 -> valを比較して小さい方のノードをcurruent -> next が指すようにする。
また採用された方のリストはnextを指すように更新し、最後にcurrentもnextを指すように更新する。

```C++
if(list1->val <= list2->val) {
    current->next = list1;
    list1 = list1->next;
} else {
    current->next = list2;
    list2 = list2->next;
}
current = current->next;
```

これをwhileループでlist1,list2の両方がnullptrでない間(どちらかがnullptrになったら打ち切る)繰り返す。以下に繰り返しの様子を載せる。

初期状態:
```mermaid
graph LR
list1 --- 1(1) --> 2(8) --> 3(9) --> 100(nullptr)
list2 --- 4(3) --> 5(5) --> 6(11) --> 200(nullptr)

101(0) --> nullptr
102(current) --- 101(0)
103(result) --- 101(0)
```

ステップ1:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 2(8) --> 3(9) --> 100(nullptr)
102(current) --- 1(1)

list1 --- 2(8)
list2 --- 4(3) --> 5(5) --> 6(11) --> 200(nullptr)
```

ステップ2:
```mermaid
graph LR
list1 --- 2(8)

103(result) --- 101(0) --> 1(1) --> 2(8) --> 3(9) --> 100(nullptr)
102(current) --- 1(1)

list2 --- 4(3) --> 5(5) --> 6(11) --> 200(nullptr)
```

ステップ3:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 2(8) --> 3(9) --> 100(nullptr)
102(current) --- 4(3)

4(3) --> 5(5) --> 6(11) --> 200(nullptr)
list2 --- 5(5)
```

ステップ4:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 2(8) --> 3(9) --> 100(nullptr)
102(current) --- 5(5)

4(3) --> 5(5) --> 6(11) --> 200(nullptr)
list2 --- 6(11)
```

ステップ5:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 3(9)
102(current) --- 2(8)

4(3) --> 5(5) --> 2(8) --> 3(9) --> 100(nullptr)
list2 --- 6(11) --> 200(nullptr)
```

ステップ6:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 100(nullptr) 
102(current) --- 3(9) --> 100(nullptr)

4(3) --> 5(5) --> 2(8) --> 3(9)
list2 --- 6(11) --> 200(nullptr)
```

ステップ7:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 100(nullptr) 
102(current) --- 3(9)

4(3) --> 5(5) --> 2(8) --> 3(9) --> 100(nullptr)
list2 --- 6(11) --> 200(nullptr)
```

ステップ8:
```mermaid
graph LR
103(result) --- 101(0) --> 1(1) --> 4(3)

list1 --- 100(nullptr) 
102(current) --- 6(11)

4(3) --> 5(5) --> 2(8) --> 3(9) --> 6(11) --> 100(nullptr)
list2 --- 200(nullptr)
```

最後にnullptrでない方のリストの要素をcurrent -> next が指すようにする。
ここまで計算してきたresultの先頭には $0$ が入っているのでこれを捨てるために

```C++
return result->next;
```

とノードを一個スキップしてからreturnする。
