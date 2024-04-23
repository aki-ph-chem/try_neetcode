# 解法のメモ

## RustのHashMap\<U,V\>とentry()

HashMap\<U,V\>でenrtry()を使うと便利だという話

参考:[Rust公式ドキュメント](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify)


- [or_insert(), or_insert_with(), or_insert_with_key(), and_modify()](impl_rust/src/hash_map/or_insert.rs)
- [or_default()](impl_rust/src/hash_map/or_default.rs)

## グラフ探索

loopによるグラフ探索(DFS,BFS)

[Rustでの実装](impl_rust/src/graph/graph_search_loop.rs)

### DFS

再帰によるDFSの色々な書き方

[Rustでの実装](impl_rust/src/graph/dfs_rec.rs)

### トポロジカルソート

DFSによるトポロジカルソート

[Rustでの実装](impl_rust/src/graph/topo_sort.rs)

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

## 部分集合、連続部分配列

- 長さ $n$ の配列の部分集合の個数: $2^n - 1$
- 長さ $n$ の配列の連続部分列の個数: $\frac{1}{2} n(n + 1)$

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
