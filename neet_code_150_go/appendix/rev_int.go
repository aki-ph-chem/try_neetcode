package main

import "fmt"

// 整数を反転させる
func rev_int(n int) int {
	rev := 0
	for n != 0 {
		tmp := n % 10
		rev = rev*10 + tmp
		n /= 10
	}

	return rev
}

// 整数でしゃくとり
func two_ptr_int(n int) {
	div := 1
	for n >= 10*div {
		div *= 10
	}

	for n > 0 {
		left := n / div
		right := n % 10

		n %= div
		n /= 10
		// n %= div, n /= 10で二桁減るため
		div /= 100

		fmt.Println("left, right = ", left, ",", right)
	}
}

func main() {
	n_1 := 123
	n_2 := 4568
	n_3 := -48

	fmt.Println("case_1: ", rev_int(n_1))
	fmt.Println("case_2: ", rev_int(n_2))
	fmt.Println("case_3: ", rev_int(n_3))

	two_ptr_int(n_1)
	two_ptr_int(n_2)
}
