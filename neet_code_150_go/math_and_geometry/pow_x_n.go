package main

import (
	"fmt"
)

// AC
func myPow(x float64, n int) float64 {
	if n >= 0 {
		return rec(x, n)
	}

	return 1.0 / rec(x, -n)
}

func rec(x float64, n int) float64 {
	if n == 1 {
		return x
	} else if n == 0 {
		return 1
	}

	tmp := rec(x, n/2)
	if n%2 == 0 {
		return tmp * tmp
	} else {
		return x * tmp * tmp
	}
}

// TLE
func myPowLoop(x float64, n int) float64 {
	if n == 0 {
		return 1
	} else if n == 1 {
		return x
	}

	power := int64(max(n, -n))
	i := int64(2)
	result := x

	for ; i <= power; i *= 2 {
		result *= result
	}
	i /= 2
	for ; i < power; i++ {
		result *= x
	}

	if n < 0 {
		return 1.0 / result
	}
	return result
}

// AC
// C++の模範解答より
func myPowCpp(x float64, n int) float64 {
	if n == 0 {
		return 1
	} else if n == 1 {
		return x
	}

	result := 1.0
	current := x
	power := max(n, -n)
	for i := power; i > 0; i /= 2 {
		if i%2 == 1 {
			result *= current
		}

		current *= current
	}

	if n < 0 {
		return 1.0 / result
	}
	return result
}

func main() {
	x_1, n_1 := 2.000, 10
	// => 1024.0000
	x_2, n_2 := 2.100, 3
	// => 9.26100
	x_3, n_3 := -3.0, -5

	fmt.Println("case_1: ", myPow(x_1, n_1))
	fmt.Println("case_2: ", myPow(x_2, n_2))
	fmt.Println("case_3: ", myPow(x_3, n_3))

	fmt.Println("case_1: ", myPowLoop(x_1, n_1))
	fmt.Println("case_2: ", myPowLoop(x_2, n_2))
	fmt.Println("case_3: ", myPowLoop(x_3, n_3))

	fmt.Println("case_1: ", myPowCpp(x_1, n_1))
	fmt.Println("case_2: ", myPowCpp(x_2, n_2))
	fmt.Println("case_3: ", myPowCpp(x_3, n_3))
}
