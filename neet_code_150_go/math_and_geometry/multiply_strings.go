package main

import "fmt"

// AC
func multiply(num1 string, num2 string) string {
	num1Byte := []byte(num1)
	num2Byte := []byte(num2)

	len_num1, len_num2 := len(num1Byte), len(num2Byte)
	result := make([]byte, len_num1+len_num2)
	set_zero(result)

	for i := len_num1 - 1; i >= 0; i-- {
		for j := len_num2 - 1; j >= 0; j-- {
			sum := (num1Byte[i]-'0')*(num2Byte[j]-'0') + (result[i+j+1] - '0')
			result[i+j+1] = sum%10 + '0'
			result[i+j] += sum / 10
		}
	}

	for i, c := range result {
		if c != '0' {
			return string(result[i:])
		}
	}

	return "0"
}

func set_zero(bytes []byte) {
	for i := 0; i < len(bytes); i++ {
		bytes[i] = '0'
	}
}

func main() {
	a_1, b_1 := "2", "3"
	// => 6
	a_2, b_2 := "123", "456"
	// => 56088
	a_3, b_3 := "12", "4"
	// => 48

	fmt.Println("case_1: ", multiply(a_1, b_1))
	fmt.Println("case_2: ", multiply(a_2, b_2))
	fmt.Println("case_3: ", multiply(a_3, b_3))
}
