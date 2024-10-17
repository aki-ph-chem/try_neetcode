package main

import "fmt"

func sum_int(array []int) int {
	sum := 0
	for _, n := range array {
		sum += n
	}

	return sum
}

func main() {
	// スライスの初期化 1
	array_1 := make([]int, 4)
	array_1[0] = 30
	array_1[1] = 12
	array_1[3] = 100

	sum_1 := sum_int(array_1)
	fmt.Printf("sum_1 = %d\n", sum_1)

	// スライスの初期化2
	array_2 := []int{1, 2, 3, 4, 5}

	sum_2 := sum_int(array_2)
	fmt.Printf("sum_2 = %d\n", sum_2)

	// 固定長の配列
	array_3 := [3]int{1, 2, 3}
	fmt.Println("array_3: ", array_3)
}
