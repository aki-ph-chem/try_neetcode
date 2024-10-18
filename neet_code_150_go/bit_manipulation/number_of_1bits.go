package main

import "fmt"

// AC
func hamingWeight(n int) int {
	result := 0
	bit := 1
	for n >= bit {
		if n&bit != 0 {
			result++
		}
		bit <<= 1
	}

	return result
}

func main() {
	case_1 := 11
	// => 3
	case_2 := 128
	// => 1
	case_3 := 2147483645
	// => 30

	fmt.Println("case_1: ", hamingWeight(case_1))
	fmt.Println("case_2: ", hamingWeight(case_2))
	fmt.Println("case_3: ", hamingWeight(case_3))
}
