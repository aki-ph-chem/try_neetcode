package main

import "fmt"

// AC
func twoSum(numbers []int, target int) []int {
	left := 0
	right := len(numbers) - 1
	for left < right {
		if target < numbers[left]+numbers[right] {
			right--
		} else if target > numbers[left]+numbers[right] {
			left++
		} else {
			return []int{left + 1, right + 1}
		}
	}

	return []int{}
}

func main() {
	case_1_array := []int{2, 7, 11, 15}
	case_1_target := 9

	fmt.Println("case_1: ", twoSum(case_1_array, case_1_target))
}
