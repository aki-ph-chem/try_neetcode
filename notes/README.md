# 解法のメモ

## gridのグラフ探索

### [Rustでの実装](impl_rust/src/graph)

### [C++での実装](impl_cpp/graph)

## 累積和

### [Rustでの実装](impl_rust/src/prefix_sum)

## ビット演算

### [ビット演算のメモ(Rust)](impl_rust/bit_manipulation)

- X OR 1
    - Xの最下位ビットを1にする

- X AND 1
    - Xの最下位ビットのみを取り出す

- X = X XOR 1
    - Xが奇数なら1を引く、偶数なら1を足す

- X AND (X - 1)
    - 0ならXは2のべき乗

- XORの応用
