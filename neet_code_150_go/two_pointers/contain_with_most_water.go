package main

// AC
func maxArea(height []int) int {
	left := 0
	right := len(height) - 1
	result := (right - left) * min(height[left], height[right])
	for left < right {
		if height[left] < height[right] {
			left++
		} else if height[left] > height[right] {
			right--
		} else {
			left++
			right--
		}
		result = max(result, (right-left)*min(height[left], height[right]))
	}

	return result
}

func main() {
}
