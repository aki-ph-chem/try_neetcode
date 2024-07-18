package main

import "fmt"

func groupAnagrams(strs []string) [][]string {
	map_s := make(map[[26]byte][]string)
	for _, s := range strs {
		table := [26]byte{}
		for _, c := range s {
			table[c-'a']++
		}

		if _, is_ex := map_s[table]; is_ex {
			map_s[table] = append(map_s[table], s)
		} else {
			map_s[table] = []string{s}
		}
	}

	result := [][]string{}
	for _, v := range map_s {
		if len(v) >= 1 {
			result = append(result, v)
		}
	}

	return result
}

func groupAnagramsAns(strs []string) [][]string {
	map_s := make(map[[26]byte][]string)
	for _, s := range strs {
		table := [26]byte{}
		for _, c := range s {
			table[c-'a']++
		}

		if _, is_ex := map_s[table]; is_ex {
			map_s[table] = append(map_s[table], s)
		} else {
			map_s[table] = []string{s}
		}
	}

	result := make([][]string, len(map_s))
	idx := 0
	for _, v := range map_s {
		result[idx] = v
		idx++
	}

	return result
}

func main() {
	array_1 := []string{"eat", "tea", "tan", "ate", "nat", "bat"}

	fmt.Println("res_1", groupAnagrams(array_1))
	fmt.Println("res_1", groupAnagramsAns(array_1))
}
