package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"strconv"
	"sync"

	"github.com/go-redis/redis/v8"
)

var ctx = context.Background()

func main() {
	key := fmt.Sprintf("go-%v.computed", os.Getenv("COMPUTE"))
	log.Printf("Redis key: %v", key)

	rdb := redis.NewClient(&redis.Options{
		Addr: "redis:6379",
	})
	batchSize, _ := strconv.Atoi(getEnv("BATCH_SIZE", "1"))
	workers, _ := strconv.Atoi(getEnv("WORKERS", "1"))
	maxCompute, _ := strconv.Atoi(getEnv("MAX_COMPUTE", "0"))

	var wg sync.WaitGroup

	runCompute := func(n int) {
		log.Printf("Starting worker [%v]", n)
		count := 0
		for {
			compute()
			count++
			if count%batchSize == 0 {
				rdb.IncrBy(ctx, key, int64(batchSize))
				log.Printf("[%v] Computed %v operations", n, count)
			}
			if maxCompute > 0 && count >= maxCompute {
				break
			}
		}
		wg.Done()
	}

	//create the go routines
	for i := 0; i < workers; i++ {
		wg.Add(1)
		go runCompute(i)
	}

	wg.Wait()

}

func getEnv(key string, defaultVal string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return defaultVal
}
