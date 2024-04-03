# 解法のメモ

## gridのグラフ探索

### [Rustでの実装](impl_rust/src/graph)

### [C++での実装](impl_cpp/graph)

## 累積和

### [Rustでの実装](impl_rust/src/prefix_sum)

## ビット演算

### [ビット演算のメモ(Rust)](impl_rust/src/bit_manipulation)

- X OR 1
    - Xの最下位ビットを1にする

- X AND 1
    - Xの最下位ビットのみを取り出す

- X = X XOR 1
    - Xが奇数なら1を引く、偶数なら1を足す

- X AND (X - 1)
    - 0ならXは2のべき乗

- Xが偶数か否かの判定

Xが奇数ならば最下位ビットは1であるので1とAND演算を行うと1になる、一方偶数の場合は最下位ビットは0であるので1とAND演算を行うと、0になる.

すなわち、X AND 1が0か否かの判定をすれば良い

- XORの応用
