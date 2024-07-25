package main

import "fmt"

// AC
func characterReplacement(s string, k int) int {
	result := 0
	left := 0
	max_f := 0
	count := make(map[byte]int)

	for right := 0; right < len(s); right++ {
		count[s[right]]++
		max_f = max(max_f, count[s[right]])

		for right-left+1-max_f > k {
			count[s[left]]--
			left++
		}

		result = max(result, right-left+1)
	}

	return result
}

func main() {
	case_1_s := "ABAB"
	case_1_k := 2

	fmt.Println("case_1: ", characterReplacement(case_1_s, case_1_k))
}
