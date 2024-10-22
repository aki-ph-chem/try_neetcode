package main

import (
	"fmt"
	"strconv"
)

// AC
// しかし、遅い
func reverse(x int) int {
	if x == 0 {
		return 0
	}

	y := max(x, -x)
	n_str := revString(strconv.Itoa(y))
	min_str_abs := strconv.Itoa(1 << 31)
	max_str := strconv.Itoa(1<<31 - 1)

	if len(n_str) == len(min_str_abs) || len(n_str) == len(max_str) {
		if n_str > min_str_abs || n_str > max_str {
			return 0
		}
	}

	return (x / y) * strNum(n_str)
}

func strNum(s string) int {
	result := 0
	base := 1
	len_s := len(s)
	for i := len_s - 1; i >= 0; i-- {
		result += base * int(s[i]-'0')
		base *= 10
	}

	return result
}

func revString(s string) string {
	bytes := make([]byte, len(s))
	for i := len(s) - 1; i >= 0; i-- {
		bytes[len(s)-1-i] = s[i]
	}

	return string(bytes)
}

// C++の模範解答より
// AC
func reverseCpp(x int) int {
	rev := 0

	const (
		INT_MAX = 1<<31 - 1
		INT_MIN = -(1 << 31)
	)

	for x != 0 {
		tmp := x % 10
		x /= 10

		if rev > INT_MAX/10 || (rev == INT_MAX/10 && tmp > 7) {
			return 0
		}
		if rev < INT_MIN/10 || (rev == INT_MIN/10 && tmp < -8) {
			return 0
		}

		rev = rev*10 + tmp
	}

	return rev
}

func main() {
	n_1 := 123
	n_2 := -123
	n_3 := 120

	fmt.Println("case_1: ", reverse(n_1))
	fmt.Println("case_2: ", reverse(n_2))
	fmt.Println("case_3: ", reverse(n_3))

	fmt.Println("case_1: ", reverseCpp(n_1))
	fmt.Println("case_2: ", reverseCpp(n_2))
	fmt.Println("case_3: ", reverseCpp(n_3))
}
