# Leet Code で出てきた英単語のメモ

- parenthesis, paren: 括弧
- palindromic: 回文の
- palindrome: 回文
- consecutive: 連続した
- polish notation: ポーランド記法
- redundant: 不必要な、冗長な
- adjacent: 隣接した
- occurrence: 存在、発生
- perimeter: 周囲の長さ
- verify: 検証する
- auxiliary: 補助の

## subarray,subsequence,substring,contiguous

配列s: {1, 1, 2, 3, 3, 4, 5}に対して

- 部分列: {1, 1, 3, 5}, {2, 4}, {1, 2, 3}, {5}

連続していなくてもいいが、順番は変わってはいけない

- 連続部分列: {1, 1, 2}, {2, 2, 3, 4}, {4}

もとの配列の連続している部分のみ

`subarray`は一般的にはこっちを指す、すなわち`contignuous subsequence`を指す。

文字列の場合は`substring`は連続部分列を指すことが多い
