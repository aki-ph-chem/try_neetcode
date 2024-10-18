package main

import "fmt"

// AC
func singleNumber(nums []int) int {
	result := 0
	for _, n := range nums {
		result ^= n
	}

	return result
}

func main() {
	case_1 := []int{2, 2, 1}
	case_2 := []int{4, 1, 2, 1, 2}
	case_3 := []int{1}

	fmt.Println("case_1: ", singleNumber(case_1))
	fmt.Println("case_2: ", singleNumber(case_2))
	fmt.Println("case_3: ", singleNumber(case_3))
}
