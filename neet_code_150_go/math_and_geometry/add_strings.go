package main

import (
	"fmt"
)

// AC
func addStrings(num1 string, num2 string) string {
	n1Bytes := []byte(num1)
	n2Bytes := []byte(num2)
	rev_bytes(n1Bytes)
	rev_bytes(n2Bytes)

	result := []byte{}
	left := []byte{}
	if len(n1Bytes) >= len(n2Bytes) {
		result = n1Bytes
		left = n2Bytes
	} else {
		result = n2Bytes
		left = n1Bytes
	}

	carry := byte(0)
	for i := 0; i < len(left); i++ {
		digit := (result[i]-'0'+left[i]-'0'+carry)%10 + '0'
		carry = (result[i] - '0' + left[i] - '0' + carry) / 10

		result[i] = digit
	}

	j := len(left)
	for carry != 0 {
		if j < len(result) {
			digit := (result[j]-'0'+carry)%10 + '0'
			carry = (result[j] - '0' + carry) / 10

			result[j] = digit
		} else {
			result = append(result, carry+'0')
			break
		}
		j++
	}

	rev_bytes(result)
	return string(result)
}

func rev_bytes(s []byte) {
	left, right := 0, len(s)-1

	for left < right {
		tmp := s[right]
		s[right] = s[left]
		s[left] = tmp
		left++
		right--
	}
}

func main() {
	a_1, b_1 := "11", "123"
	a_2, b_2 := "456", "77"

	fmt.Println("case_1: ", addStrings(a_1, b_1))
	fmt.Println("case_2: ", addStrings(a_2, b_2))
}
