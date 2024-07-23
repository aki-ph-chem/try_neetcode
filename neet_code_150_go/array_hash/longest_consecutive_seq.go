package main

import "fmt"

// AC
func longestConsecutive(nums []int) int {
	set := make(map[int]bool)
	for _, n := range nums {
		set[n] = true
	}

	result := 0
	for n := range set {
		if !set[n-1] {
			tmp := n
			current := 0
			for set[tmp] {
				tmp++
				current++
			}
			result = max(result, current)
		}
	}

	return result
}

// AC
// こっちのほうが遅い
func longestConsecutive2(nums []int) int {
	set := make(map[int]bool)
	for _, n := range nums {
		set[n] = true
	}

	result := 0
	for _, n := range nums {
		if !set[n-1] {
			tmp := n
			current := 0
			for set[tmp] {
				tmp++
				current++
			}
			result = max(result, current)
		}
	}

	return result
}

func main() {
	case_1 := []int{100, 4, 200, 1, 3, 2}
	case_2 := []int{0, 3, 7, 2, 5, 8, 4, 6, 0, 1}

	fmt.Println("case_1: ", longestConsecutive(case_1))
	fmt.Println("case_2: ", longestConsecutive(case_2))

	fmt.Println("case_1: ", longestConsecutive2(case_1))
	fmt.Println("case_2: ", longestConsecutive2(case_2))
}
