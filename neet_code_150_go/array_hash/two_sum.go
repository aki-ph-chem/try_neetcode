package main

import "fmt"

func twoSumeSq(nums []int, target int) []int {
	result := []int{}
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				result = append(result, i)
				result = append(result, j)
				return result
			}
		}
	}

	return result
}

func twoSumeLine(nums []int, target int) []int {
	map_s := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		diff := target - nums[i]
		if idx_diff, is_ex := map_s[diff]; is_ex {
			return []int{idx_diff, i}
		} else {
			map_s[nums[i]] = i
		}
	}

	return []int{}
}

func twoSumeAns(nums []int, target int) []int {
	map_s := make(map[int]int)
	for idx, num := range nums {
		diff := target - num
		if idx_diff, is_ex := map_s[diff]; is_ex {
			return []int{idx_diff, idx}
		} else {
			map_s[num] = idx
		}
	}

	return []int{}
}

func main() {
	nums_1 := []int{2, 7, 11, 15}
	target_1 := 9

	nums_2 := []int{3, 2, 4}
	target_2 := 6

	nums_3 := []int{3, 3}
	target_3 := 6

	fmt.Println("nums_1:", twoSumeSq(nums_1, target_1))
	fmt.Println("nums_2:", twoSumeSq(nums_2, target_2))
	fmt.Println("nums_3:", twoSumeSq(nums_3, target_3))

	fmt.Println("nums_1:", twoSumeLine(nums_1, target_1))
	fmt.Println("nums_2:", twoSumeLine(nums_2, target_2))
	fmt.Println("nums_3:", twoSumeLine(nums_3, target_3))

	fmt.Println("nums_1:", twoSumeAns(nums_1, target_1))
	fmt.Println("nums_2:", twoSumeAns(nums_2, target_2))
	fmt.Println("nums_3:", twoSumeAns(nums_3, target_3))
}
