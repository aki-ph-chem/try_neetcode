package main

import "fmt"

// AC: 遅い:O(N^2)
func lengthOfLongestSubstring(s string) int {
	result := 0
	left := 0
	right := 0

	for right < len(s) {
		current := 0
		set := make(map[byte]bool)
		for right < len(s) && !set[s[right]] {
			set[s[right]] = true
			current++
			right++
		}
		left++
		right = left
		result = max(result, current)
	}

	return result
}

// AC
func lengthOfLongestSubstring2(s string) int {
	result := 0
	left := 0
	right := 0
	set := make(map[byte]bool)
	for right < len(s) {
		for set[s[right]] {
			set[s[left]] = false
			left++
		}

		result = max(result, right-left+1)
		set[s[right]] = true
		right++
	}

	return result
}

func main() {
	case_1 := "abcabcbb"
	// => 3
	case_2 := "bbbbb"
	// => 1
	case_3 := "pwwkew"
	// => 3
	case_4 := "dvdf"
	// => 3

	fmt.Println("case_1: ", lengthOfLongestSubstring(case_1))
	fmt.Println("case_2: ", lengthOfLongestSubstring(case_2))
	fmt.Println("case_3: ", lengthOfLongestSubstring(case_3))
	fmt.Println("case_4: ", lengthOfLongestSubstring(case_4))

	fmt.Println("case_1: ", lengthOfLongestSubstring2(case_1))
	fmt.Println("case_2: ", lengthOfLongestSubstring2(case_2))
	fmt.Println("case_3: ", lengthOfLongestSubstring2(case_3))
	fmt.Println("case_4: ", lengthOfLongestSubstring2(case_4))
}
