package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/go-redis/redis/v8"
)

var ctx = context.Background()

func main() {
	key := fmt.Sprintf("go-%v.computed", os.Getenv("COMPUTE"))
	log.Printf("Redis key: %v", key)

	rdb := redis.NewClient(&redis.Options{
		Addr: "redis:6379",
	})
	batchSize, _ := strconv.Atoi(os.Getenv("BATCH_SIZE"))
	count := 0
	for {
		compute()
		count++
		if count%batchSize == 0 {
			rdb.IncrBy(ctx, key, int64(batchSize))
			log.Printf("Computed %v operations", count)
		}
	}
}
