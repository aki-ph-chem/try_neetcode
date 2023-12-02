# NeetCode 150

取り組む際の指針:
- 15分考えてわからなかったら答えをみる 
- Leet Codeに課金しないと見れない問題は無視しても良しとする

## Array & Hashing

1. Contains Duplicate
    - 初見: x  
    - 理解: o

2. Valid Anagram 
    - 初見: o  
    - 理解: o

3. Two Sum
    - 初見: o
    - 理解: o

4. Group Anagrams
    - 初見: x
    - 理解: o

5. Top K Frequent Elements
    - 初見: x
    - 理解: x

6. Product of Array Except Self
    - 初見: x
    - 理解: ○

7. Valid Sudoku: そもそも題意を間違って認識していた(その数独が解けるのかの判定ではなく、現在の段階で数独のルールを満たしているかさえ判定できればよい)
    - 初見: x
    - 理解: x

8. 要課金なため略

9. Longest Consective Sequence
    - 初見: x
    - 理解: o 

## Two Pointers 

10. Valid Palindrome
    - 初見: o
    - 理解: o

11. Two Sum II
    - 初見: o: アクセプトされたけど模範解答と比べて二桁ほど遅い
    - 理解: o

12. 3sum
    - 初見: x
    - 理解: x

13. Continer with Most Water
    - 初見: x 
    - 理解: o

14. Trapping Rain Water : Hardなため一旦飛ばす

## Sliding Window 

15. Best Time to Buy And Sell Stock 
    - 初見: x
    - 理解: x

16. Longest Substring Without Repeating Characters
    - 初見: o
    - 理解: o

17. Longest Repeating Character Replacement
    - 初見: x
    - 理解: x

18. Permutation In String
    - 初見: x
    - 理解: x

19. Minimum Window Substring: Hardなため一旦飛ばす

20. Sliding Window Maximum: Hardなため一旦飛ばす

## Stack

21. Valid Parentheses
    - 初見: △(類題をけんちょ本で見たため)
    - 理解: o

22. Min Stack 
    - 初見: o
    - 理解: o

23. Evaluate Reverse Polish Notation 
    - 初見: △(類題をけんちょ本で見たため)
    - 理解: o

24.  Generate Parentheses
    - 初見: x
    - 理解: o

25.  Daily Temperatures
    - 初見: x
    - 理解: o

26.  Car Fleet 
    - 初見: x
    - 理解: o

27. Largest Rectangle In Histogram: Hardなため一旦飛ばす

## Binary Search

28. Binry Search
    - 初見: o
    - 理解: o

29. Seach a 2D Matrix
    - 初見: o
    - 理解: o

30. Koko Eating Bananas 
    - 初見: x
    - 理解: x

31. Find Minimum In Rotated Sorted Array
    - 初見: o
    - 理解: o

32. Search In Rotated Sorted Array
    - 初見: x
    - 理解: o

33. Time Based Key Value Store
    - 初見: o 
    - 理解: o

34. Median of Two Sorted Arrays: Hardなため一旦飛ばす

## Linked List

35. Reverse Linked List 
    - 初見: x 
    - 理解: o

36. Merge Two Sorted Lists 
    - 初見: x 
    - 理解: o 

## Greedy

888. Maximum Subarray
    - 初見: x
    - 理解: x

888. Jump Game
    - 初見: x
    - 理解: o

## Math & Geometry

9999. Rotate Image
    - 初見: x
    - 理解: o

9999. Spiral Matrix 
    - 初見: x
    - 理解: o

9999. Set Matrix Zeroes 
    - 初見: x
    - 理解: o

9999. Happy Number 
    - 初見: o
    - 理解: x

9999. Plus One 
    - 初見: o
    - 理解: x 

9999. Pow 
    - 初見: o
    - 理解: x 

9999. Multiply Strings 
    - 初見: x
    - 理解: o 

9999. Detect Squares
    - 初見: x
    - 理解: o 


## 解法などのメモ

## Search In Rotated Sorted Array

まず、中点が右側のソートされた領域に落ちるか、左側のソートされた領域に落ちるか場合分けする。
次に中点とtargetの大小関係で場合わけしてlもしくはrを更新する
