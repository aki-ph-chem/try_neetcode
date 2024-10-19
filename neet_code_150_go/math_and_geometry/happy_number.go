package main

import "fmt"

// AC
func isHappy(n int) bool {
	set := make(map[int]bool)
	num := 0
	for {
		for n > 0 {
			r := n % 10
			num += r * r
			n /= 10
		}

		if num == 1 {
			return true
		} else if set[num] {
			return false
		}

		set[num] = true
		n = num
		num = 0

	}
}

func main() {
	n_1 := 19
	n_2 := 2

	fmt.Println("case_1: ", isHappy(n_1))
	fmt.Println("case_2: ", isHappy(n_2))
}
