package main

import (
	"os"
	"sort"
	"strconv"
)

func compute() {

	// size of the array to sort
	size, _ := strconv.Atoi(os.Getenv("ARRAY_SIZE"))

	//create an input array with unsorted values
	input := make([]int, size)
	incr := 1
	for i := 1; i < size; i++ {
		input[i] = -input[i-1] + incr
		incr = -incr
	}

	// sort the array
	sort.Ints(input)
}
