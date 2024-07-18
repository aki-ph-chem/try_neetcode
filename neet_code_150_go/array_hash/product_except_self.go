package main

import (
	"fmt"
)

func ProductExceptSelf(nums []int) []int {
	n := len(nums)
	prefix_prod := make([]int, n+1)
	prefix_prod[0] = 1
	for i := 0; i < n; i++ {
		prefix_prod[i+1] = prefix_prod[i] * nums[i]
	}

	result := make([]int, n)
	current := 1
	for i := n - 1; i >= 0; i-- {
		result[i] = prefix_prod[i] * current
		current *= nums[i]
	}

	return result
}

func main() {
	nums_1 := []int{1, 2, 3, 4}

	res_1 := ProductExceptSelf(nums_1)
	fmt.Println("res_1", res_1)
}
