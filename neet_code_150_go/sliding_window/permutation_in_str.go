package main

import "fmt"

// AC
func checkInclusion(s1 string, s2 string) bool {
	n_1 := len(s1)
	n_2 := len(s2)
	if n_1 > n_2 {
		return false
	}

	count_map := [26]int{}

	// 0 ~ n_1 - 1
	for i := 0; i < n_1; i++ {
		count_map[s1[i]-'a'] += 1
		count_map[s2[i]-'a'] -= 1
	}
	if isPermutation(count_map) {
		return true
	}

	for i := n_1; i < n_2; i++ {
		count_map[s2[i]-'a'] -= 1
		count_map[s2[i-n_1]-'a'] += 1
		if isPermutation(count_map) {
			return true
		}
	}

	return false
}

func isPermutation(count [26]int) bool {
	for i := 0; i < 26; i++ {
		if count[i] != 0 {
			return false
		}
	}

	return true
}

// mapを用いた実装
// AC
func checkInclusionMap(s1 string, s2 string) bool {
	n_1 := len(s1)
	n_2 := len(s2)
	if n_1 > n_2 {
		return false
	}

	count_map := make(map[byte]int)

	// 0 ~ n_1 - 1
	for i := 0; i < n_1; i++ {
		count_map[s1[i]] += 1
		count_map[s2[i]] -= 1
	}
	if isPermutationMap(count_map) {
		return true
	}

	for i := n_1; i < n_2; i++ {
		count_map[s2[i]] -= 1
		count_map[s2[i-n_1]] += 1
		if isPermutationMap(count_map) {
			return true
		}
	}

	return false
}

func isPermutationMap(count map[byte]int) bool {
	for _, n := range count {
		if n != 0 {
			return false
		}
	}

	return true
}

func main() {
	s_1, s_2 := "ab", "eibaooo"

	res := checkInclusion(s_1, s_2)
	fmt.Printf("res = %d\n", res)
	res = checkInclusionMap(s_1, s_2)
	fmt.Printf("res = %d\n", res)
}
