package main

import "fmt"

func containsDuplicate(nums []int) bool {
	set := make(map[int]bool)

	for _, n := range nums {
		if set[n] {
			return true
		} else {
			set[n] = true
		}
	}

	return false
}

func containsDuplicate2(nums []int) bool {
	set := make(map[int]struct{})

	for _, n := range nums {
		if _, ok := set[n]; ok {
			return true
		}
		set[n] = struct{}{}

	}

	return false
}

func main() {
	nums_1 := []int{1, 2, 1, 3}
	nums_2 := []int{1, 2, 3, 4}
	nums_3 := []int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}

	fmt.Println("nums_1", containsDuplicate(nums_1))
	fmt.Println("nums_2", containsDuplicate(nums_2))
	fmt.Println("nums_3", containsDuplicate(nums_3))

	fmt.Println("nums_1", containsDuplicate2(nums_1))
	fmt.Println("nums_2", containsDuplicate2(nums_2))
	fmt.Println("nums_3", containsDuplicate2(nums_3))
}
