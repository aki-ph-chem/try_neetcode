package main

import "fmt"

func isAnagram(s string, t string) bool {
	if s == t {
		return true
	} else if len(s) != len(t) {
		return false
	}

	map_s := make(map[byte]int)
	map_t := make(map[byte]int)

	for i := 0; i < len(s); i++ {
		if map_s[s[i]] >= 1 {
			map_s[s[i]] += 1
		} else {
			map_s[s[i]] = 1
		}

		if map_t[t[i]] >= 1 {
			map_t[t[i]] += 1
		} else {
			map_t[t[i]] = 1
		}

	}

	for i := 0; i < len(s); i++ {
		if map_s[s[i]] != map_t[s[i]] {
			return false
		}
	}

	return true
}

func isAnagram2(s string, t string) bool {
	if s == t {
		return true
	} else if len(s) != len(t) {
		return false
	}

	freq := [26]int{}
	for i := 0; i < len(s); i++ {
		freq[s[i]-'a']++
		freq[s[i]-'a']--
	}

	for i := 0; i < len(freq); i++ {
		if freq[i] != 0 {
			return false
		}
	}

	return true
}

func main() {
	s_1 := "anagram"
	t_1 := "nagaram"

	fmt.Println("res_1", isAnagram(s_1, t_1))
	fmt.Println("res_1", isAnagram2(s_1, t_1))
}
