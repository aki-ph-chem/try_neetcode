package main

import "fmt"

// C++の模範解答より
// AC
func getSumCpp(a int, b int) int {
	for b != 0 {
		carry := a & b
		a ^= b
		b = carry << 1
	}

	return a
}

func main() {
	a_1, b_1 := 1, 2
	// => 3
	a_2, b_2 := 2, 3
	// => 5

	fmt.Println("case_1: ", getSumCpp(a_1, b_1))
	fmt.Println("case_2: ", getSumCpp(a_2, b_2))
}
