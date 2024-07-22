package main

// AC
func isPalindrome(s string) bool {
	isAlphanumeric := func(c byte) bool {
		const (
			num_start   = '0'
			num_end     = '9'
			small_start = 'a'
			small_end   = 'z'
			big_start   = 'A'
			big_end     = 'Z'
		)

		if (num_start <= c && c <= num_end) || (small_start <= c && c <= small_end) || (big_start <= c && c <= big_end) {
			return true
		}

		return false
	}
	toLowercase := func(c byte) byte {
		const (
			big_start = 'A'
			big_end   = 'Z'
		)
		if c < big_start || big_end < c {
			return c
		}

		return 'a' + (c - big_start)
	}

	left := 0
	right := len(s) - 1

	for left < right {
		for !isAlphanumeric(s[left]) && left < right {
			left++
		}
		for !isAlphanumeric(s[right]) && left < right {
			right--
		}
		if toLowercase(s[left]) != toLowercase(s[right]) {
			return false
		}

		left++
		right--
	}

	return true
}

func main() {
}
