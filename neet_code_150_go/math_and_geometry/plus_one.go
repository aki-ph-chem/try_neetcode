package main

import "fmt"

// AC
func plusOne(digits []int) []int {
	revArray(digits)
	carry := 1
	for i := 0; i < len(digits); i++ {
		dgt := (digits[i] + carry) % 10
		carry = (digits[i] + carry) / 10

		digits[i] = dgt
	}
	if carry != 0 {
		digits = append(digits, carry)
	}

	revArray(digits)
	return digits
}

func revArray(array []int) {
	left, right := 0, len(array)-1

	for left < right {
		tmp := array[left]
		array[left] = array[right]
		array[right] = tmp

		left++
		right--
	}
}

// reverseを用いない
// AC
func plusOne2(digits []int) []int {
	carry := 1
	for i := len(digits) - 1; i >= 0; i-- {
		dgt := (digits[i] + carry) % 10
		carry = (digits[i] + carry) / 10

		digits[i] = dgt
	}

	if carry == 0 {
		return digits
	}

	result := make([]int, len(digits)+1)
	result[0] = carry
	for i := 0; i < len(digits); i++ {
		result[i+1] = digits[i]
	}

	return result
}

// C++の模範解答
func plusOneCpp(digits []int) []int {
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i] < 9 {
			digits[i]++
			return digits
		}
		digits[i] = 0
	}

	// 全部の数字が9だった場合
	digits[0] = 1
	digits = append(digits, 0)

	return digits
}

func main() {
	case_1 := []int{1, 2, 3}
	case_2 := []int{4, 3, 2, 1}
	case_3 := []int{9}

	fmt.Println("case_1: ", plusOne(case_1))
	fmt.Println("case_2: ", plusOne(case_2))
	fmt.Println("case_3: ", plusOne(case_3))

	case_1 = []int{1, 2, 3}
	case_2 = []int{4, 3, 2, 1}
	case_3 = []int{9}

	fmt.Println("case_1: ", plusOne2(case_1))
	fmt.Println("case_2: ", plusOne2(case_2))
	fmt.Println("case_3: ", plusOne2(case_3))

	case_1 = []int{1, 2, 3}
	case_2 = []int{4, 3, 2, 1}
	case_3 = []int{9}

	fmt.Println("case_1: ", plusOneCpp(case_1))
	fmt.Println("case_2: ", plusOneCpp(case_2))
	fmt.Println("case_3: ", plusOneCpp(case_3))
}
