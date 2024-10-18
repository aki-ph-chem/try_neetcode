package main

import "fmt"

// AC
func missingNumber(nums []int) int {
	result := 0
	for i, n := range nums {
		result ^= n
		result ^= (i + 1)
	}

	return result
}

func main() {
	case_1 := []int{3, 0, 1}
	// => 2
	case_2 := []int{0, 1}
	// => 2
	case_3 := []int{9, 6, 4, 2, 3, 5, 7, 0, 1}
	// => 8

	fmt.Println("case_1: ", missingNumber(case_1))
	fmt.Println("case_2: ", missingNumber(case_2))
	fmt.Println("case_3: ", missingNumber(case_3))
}
