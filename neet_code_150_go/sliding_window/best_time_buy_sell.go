package main

import "fmt"

// AC
func maxProfit(prices []int) int {
	left := 0
	right := 0
	result := 0

	for right < len(prices) {
		if prices[right] > prices[left] {
			result = max(result, prices[right]-prices[left])
		} else {
			left = right
		}
		right++
	}

	return result
}

// AC
func maxProfit2(prices []int) int {
	left := 0
	right := 0
	result := 0

	for right < len(prices) {
		for right < len(prices) && prices[right] > prices[left] {
			result = max(result, prices[right]-prices[left])
			right++
		}
		left = right
		right++
	}

	return result
}

func main() {
	case_1 := []int{7, 1, 5, 3, 6, 4}
	case_2 := []int{7, 6, 4, 3, 1}

	fmt.Println("case_1: ", maxProfit(case_1))
	fmt.Println("case_2: ", maxProfit(case_2))

	fmt.Println("case_1: ", maxProfit2(case_1))
	fmt.Println("case_2: ", maxProfit2(case_2))
}
