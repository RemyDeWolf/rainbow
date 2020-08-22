package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math/rand"
	"os"
	"strconv"
)

func compute() {

	writes, _ := strconv.Atoi(os.Getenv("WRITES"))
	reads, _ := strconv.Atoi(os.Getenv("READS"))
	fileSize, _ := strconv.Atoi(os.Getenv("FILE_SIZE"))

	//create the data array
	data := make([]byte, fileSize)

	// write the files
	for i := 0; i < writes; i++ {
		path := fmt.Sprintf("/data/%v", i)
		err := ioutil.WriteFile(path, data, 0644)
		if err != nil {
			log.Fatal(err)
		}
	}

	//read the files
	for i := 0; i < reads; i++ {
		path := fmt.Sprintf("/data/%v", rand.Intn(writes))
		_, err := ioutil.ReadFile(path)
		if err != nil {
			panic(err)
		}
	}

}
