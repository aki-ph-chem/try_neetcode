package main

import "slices"

// AC
func treeSum(nums []int) [][]int {
	slices.Sort(nums)
	n := len(nums)

	result := [][]int{}
	for i := 0; i < n-2; i++ {
		if nums[i] > 0 {
			break
		}
		if i > 0 && nums[i-1] == nums[i] {
			continue
		}

		left := i + 1
		right := n - 1

		for left < right {
			sum := nums[i] + nums[left] + nums[right]

			if sum < 0 {
				left++
			} else if sum > 0 {
				right--
			} else {
				result = append(result, []int{nums[i], nums[left], nums[right]})

				for left < right && nums[left] == nums[left+1] {
					left++
				}
				left++
				for left < right && nums[right] == nums[right-1] {
					right--
				}
				right--
			}
		}
	}

	return result
}

func main() {
}
