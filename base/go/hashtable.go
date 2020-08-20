package main

import (
	"math/rand"
	"os"
	"strconv"

	"github.com/google/uuid"
)

func compute() {

	size, _ := strconv.Atoi(os.Getenv("HASHTABLE_SIZE"))
	readCount, _ := strconv.Atoi(os.Getenv("READ_COUNT"))

	//create an input hash table and an array
	hashtable := make(map[uuid.UUID]uuid.UUID, size)
	keys := make([]uuid.UUID, size)

	// init the hash table
	for i := 0; i < size; i++ {
		key := uuid.New()
		hashtable[key] = key
		keys[i] = key
	}

	//access hash table
	for i := 0; i < readCount; i++ {
		key := keys[rand.Intn(size)]
		_ = hashtable[key]
	}

}
