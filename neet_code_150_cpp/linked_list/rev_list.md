# Linked List 解法メモ

## Reverse Linked List

元の状態

```mermaid
graph LR
1(1) --> 2(2) --> 3(3) --> 4(nullptr)
```

### 初期化

```C++
auto node_p = heda;
auto node_c = head->next; 
node_p->next = nullptr;
```

先頭であるノード $1$ へのポインタをnode\_pに入れ、二番目のノード$2$へのポインタをnode\_c に入れる。このときnode\_pのnextはnullptrを入れる。


このときnode\_p, node\_cは以下のように変化を受ける。

```C++
auto node_p = heda;
auto node_c = head->next; 
```
```mermaid
graph LR
1(1) --> 2(2) --> 3(3) --> 4(nullptr)
10(node_p) --- 1(1)
11(node_c) --- 2(2)
```
```C++
node_p->next = nullptr;
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
2(2) --> 3(3) --> 4(nullptr)
10(node_p) --- 1(1)
11(node_c) --- 2(2)
```

### ポインタの付替え

```C++
auto node_tmp = node_c->next;
node_c->next = node_p;
node_p = node_c;
node_c = node_tmp;
```

まずnode\_cのnext(ノード $2$ の次のノード)をnode\_tmpに一時的に持たせる。

続いて、ノード $2$ へのポインタを格納しているnode\_cのnextがノードに $1$ へのポインタを格納しているnode\_pへのポインタを格納する

最後にnode\_pがnode\_cを指すように、node\_cがnode\_tmpを指すように更新する。これをnode\_c -> nextがnullptrでない間繰り返す(while文で繰り返す)。

このときのノードの変化とC++のコードの対応は以下のようになる。

```C++
auto node_tmp = node_c->next;
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
2(2) --> 3(3) --> 4(nullptr)
13(node_tmp) --- 3(3)
10(node_p) --- 1(1)
11(node_c) --- 2(2)
```

```C++
node_c->next = node_p;
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
3(3) --> 4(nullptr)
2(2) --> 1(1)
13(node_tmp) --- 3(3)
10(node_p) --- 1(1)
11(node_c) --- 2(2)
```

```C++
node_p = node_c
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
3(3) --> 4(nullptr)
2(2) --> 1(1)
13(node_tmp) --- 3(3)
10(node_p) --- 2(2)
11(node_c) --- 2(2)
```

```C++
node_c = node_tmp
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
3(3) --> 4(nullptr)
2(2) --> 1(1)
13(node_tmp) --- 3(3)
10(node_p) --- 2(2)
11(node_c) --- 3(3)
```

今のnode\_cのではnode\_c -> next = nullptrであるのでwhileループを抜けて

```C++
node_c->next = node_p;
```
```mermaid
graph LR
1(1) --> 12(nullptr) 
3(3) --> 2(2)
2(2) --> 1(1)
13(node_tmp) --- 3(3)
10(node_p) --- 2(2)
11(node_c) --- 3(3)
```
とnode\_cの次のノードを書き換えて,最後にnode\_cへのポインタをretunして終了する。
