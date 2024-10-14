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

func checkInclusion2(s1 string, s2 string) bool {
	n_1 := len(s1)
	n_2 := len(s2)
	if n_1 > n_2 {
		return false
	}

	count_map := make([]int, 26)

	// 0 ~ n_1 - 1
	for i := 0; i < n_1; i++ {
		count_map[s1[i]-'a'] += 1
		count_map[s2[i]-'a'] -= 1
	}
	if isPermutation2(count_map) {
		return true
	}

	for i := n_1; i < n_2; i++ {
		count_map[s2[i]-'a'] -= 1
		count_map[s2[i-n_1]-'a'] += 1
		if isPermutation2(count_map) {
			return true
		}
	}

	return false
}

// AC
// 配列を二つ使う方法
func checkInclusion3(s1 string, s2 string) bool {
	n_1 := len(s1)
	n_2 := len(s2)
	if n_1 > n_2 {
		return false
	}

	count_map_s1 := make([]int, 26)
	count_map_s2 := make([]int, 26)

	// i:  0 ~ n_1 - 1
	for i := 0; i < n_1; i++ {
		count_map_s1[s1[i]-'a'] += 1
		count_map_s2[s2[i]-'a'] += 1
	}
	if isSmaeCount(count_map_s1, count_map_s2) {
		return true
	}
	// fmt.Println("count_map_s1", count_map_s1)

	// i:  n_1 ~ n_2 - 1
	for i := n_1; i < n_2; i++ {
		count_map_s2[s2[i-n_1]-'a'] -= 1
		count_map_s2[s2[i]-'a'] += 1
		// fmt.Println("count_map_s2", count_map_s2)
		if isSmaeCount(count_map_s1, count_map_s2) {
			return true
		}
	}

	return false
}

func isSmaeCount(count_1, count_2 []int) bool {
	for i := 0; i < 26; i++ {
		if count_1[i] != count_2[i] {
			return false
		}
	}

	return true
}

func isPermutation2(count []int) bool {
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

	res = checkInclusion2(s_1, s_2)
	fmt.Printf("res = %d\n", res)

	res = checkInclusion3(s_1, s_2)
	fmt.Printf("res = %d\n", res)

	s_1, s_2 = "ab", "eidboaoo"

	res = checkInclusion2(s_1, s_2)
	fmt.Printf("res = %d\n", res)

	res = checkInclusion3(s_1, s_2)
	fmt.Printf("res = %d\n", res)
}
