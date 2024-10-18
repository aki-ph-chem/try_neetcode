package main

import "fmt"

// AC
func countBits(n int) []int {
	num_1 := func(n int) int {
		n_1 := 0
		bit := 1
		for n >= bit {
			if n&bit != 0 {
				n_1++
			}

			bit <<= 1
		}

		return n_1
	}

	result := make([]int, n+1)
	for i := 0; i <= n; i++ {
		result[i] = num_1(i)
	}

	return result
}

func main() {
	case_1 := 2
	case_2 := 5

	fmt.Println("case_1: ", countBits(case_1))
	fmt.Println("case_2: ", countBits(case_2))
}
