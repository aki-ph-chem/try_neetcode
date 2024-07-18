package main

import "fmt"

func topKFrequent(nums []int, k int) []int {
	map_n := make(map[int]int)
	for _, n := range nums {
		map_n[n]++
	}

	table := make([][]int, len(nums)+1)
	for key, v := range map_n {
		table[v] = append(table[v], key)
	}
	result := []int{}
	for i := len(table) - 1; i >= 0; i-- {
		if len(table[i]) != 0 {
			j := 0
			for k > 0 && j < len(table[i]) {
				result = append(result, table[i][j])
				k--
				j++
			}
		}
	}

	return result
}

func topKFrequentAns(nums []int, k int) (res []int) {
	countMap := map[int]int{}
	countSlice := make([][]int, len(nums)+1)

	for _, num := range nums {
		if count, ok := countMap[num]; ok {
			countMap[num] = count + 1
		} else {
			countMap[num] = 1
		}
	}

	for num, count := range countMap {
		countSlice[count] = append(countSlice[count], num)
	}

	for i := len(countSlice) - 1; i > 0; i-- {
		res = append(res, countSlice[i]...)
		if len(res) == k {
			return
		}
	}

	return
}

func main() {
	nums_1 := []int{1, 1, 1, 2, 2, 3}
	k_1 := 2

	fmt.Println("nums_1:", topKFrequent(nums_1, k_1))
	fmt.Println("nums_1:", topKFrequentAns(nums_1, k_1))
}
