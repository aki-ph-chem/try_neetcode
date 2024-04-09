# 解法のメモ

## gridのグラフ探索

[Rustでの実装](impl_rust/src/graph/graph_grid.rs)

[C++での実装](impl_cpp/graph/graph_grid.cpp)

### flood fill アルゴリズム

[Rustでの実装](impl_rust/src/graph/flood_fill.rs)

お絵描きソフト塗りつぶし機能で使われているらしい

- 参考
    - [Flood fill Algorithm – how to implement fill() in paint?](https://www.geeksforgeeks.org/flood-fill-algorithm-implement-fill-paint/) 

## 累積和

[Rustでの実装](impl_rust/src/prefix_sum/prefix_sum.rs)

## ビット演算

- X OR 1
    - Xの最下位ビットを1にする

- X AND 1
    - Xの最下位ビットのみを取り出す

- X = X XOR 1
    - Xが奇数なら1を引く、偶数なら1を足す

- X AND (X - 1)
    - 0ならXは2のべき乗

[実装(1)(Rust)](impl_rust/src/bit_manipulation/bit_1.rs)

- Xが偶数か否かの判定

Xが奇数ならば最下位ビットは1であるので1とAND演算を行うと1になる、一方偶数の場合は最下位ビットは0であるので1とAND演算を行うと、0になる.

すなわち、X AND 1が0か否かの判定をすれば良い

[実装(2)(Rust)](impl_rust/src/bit_manipulation/bit_2.rs)

- XORの応用
