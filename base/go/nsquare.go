package main

import (
	"os"
	"strconv"
)

func compute() {

	// size of the array to compute
	size, _ := strconv.Atoi(os.Getenv("NSQUARE_ARRAY_SIZE"))

	//create an input slice
	input := make([]int, size)

	// n square read/write operation
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			input[j] = (input[i] + j) / 2
		}
	}
}
