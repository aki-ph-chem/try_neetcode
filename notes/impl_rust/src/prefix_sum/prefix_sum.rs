fn main() {
    let array = vec![1, 5, 3, 10, 5];

    // prefix_sum[i]: i - 1 までの累積和
    let mut prefix_sum = vec![0; array.len() + 1];
    prefix_sum[0] = 0;
    for i in 0..array.len() {
        prefix_sum[i + 1] = array[i] + prefix_sum[i];
    }

    println!("prefix_sum: {:?}", prefix_sum);

    // 閉区間[i,k] (i < k)の和の計算は愚直に実装するとO(N)
    // 複数回異なる閉区間の和を求めるなら定数時間で計算できる累積和の方法を使うと良い(ただし、累積和の配列の作成にO(N)かかることに注意)

    // 閉区間[i,k]の和: prefix_sum[k + 1] - prefix_sum[i]
    println!("sum([0,2]): {}", prefix_sum[2 + 1] - prefix_sum[0]);
    // => 9
    println!("sum([2,3]): {}", prefix_sum[3 + 1] - prefix_sum[2]);
    // => 13

    // なお左閉右開区間[i,k)の和は prefix_sum[k] - prefix_sum[i]で計算できる
    println!("sum([0,2)): {}", prefix_sum[2] - prefix_sum[0]);
    // => 6
    println!("sum([2,3)): {}", prefix_sum[3] - prefix_sum[2]);
    // => 3
}
