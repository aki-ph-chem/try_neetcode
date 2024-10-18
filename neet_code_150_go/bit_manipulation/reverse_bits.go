package main

import "fmt"

// AC
func reverseBits(num uint32) uint32 {
	bit_list := [32]int{}
	for i := 0; i < 32; i++ {
		if num&(1<<i) != 0 {
			bit_list[i] = 1
		}
	}
	// fmt.Println("bit_list: ", bit_list)

	result := uint32(0)
	for i := 31; i >= 0; i-- {
		result += uint32(bit_list[i]) * (1 << (31 - i))
	}

	return result
}

// AC
func reverseBits2(num uint32) uint32 {
	result := uint32(0)
	for i := 31; i >= 0; i-- {
		if num&(1<<i) != 0 {
			result += 1 << (31 - i)
		}
	}

	return result
}

// AC
// 模範解答より
func reverseBits3(num uint32) uint32 {
	result := uint32(0)
	for i := 0; i < 32; i++ {
		result = (result << 1) | (num & 1)
		num >>= 1
	}

	return result
}

func main() {
	case_1 := uint32(0b00000010100101000001111010011100)
	// => 00111001011110000010100101000000
	case_2 := uint32(0b11111111111111111111111111111101)
	// => 10111111111111111111111111111111

	fmt.Printf("case_1: %b\n", reverseBits(case_1))
	fmt.Printf("case_2: %b\n", reverseBits(case_2))

	fmt.Printf("case_1: %b\n", reverseBits2(case_1))
	fmt.Printf("case_2: %b\n", reverseBits2(case_2))

	fmt.Printf("case_1: %b\n", reverseBits3(case_1))
	fmt.Printf("case_2: %b\n", reverseBits3(case_2))
}
