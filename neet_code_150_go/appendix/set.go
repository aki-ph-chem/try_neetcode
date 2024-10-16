package main

import "fmt"

func main() {
	// Goでは、setがないのでmapで代用する
	// int
	set_int := make(map[int]bool)
	set_int[1] = true
	set_int[12] = true
	set_int[31] = true
	for key := range set_int {
		fmt.Printf("key: %d\n", key)
	}

	// string
	set_string := make(map[string]bool)
	set_string["foo"] = true
	set_string["hoge"] = true
	for key := range set_string {
		fmt.Printf("key: %s\n", key)
	}
}
