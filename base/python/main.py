import logging
import os
import redis

import compute

def main():
    logging.basicConfig(level=logging.INFO, 
        format='%(asctime)s %(levelname)s %(message)s', 
        datefmt='%Y-%m-%d %H:%M:%S')

    redisClient = redis.Redis(host='redis', port=6379, decode_responses=True)

    base = os.environ['BASE']
    key = '{}-{}.computed'.format(base, os.environ['COMPUTE'])
    logging.info("Redis key: {}".format(key))

    batchSize = int(os.environ['BATCH_SIZE'])
    count = 0
    while True:
        compute.compute()
        count+=1
        if count%batchSize == 0:
            redisClient.incr(key, batchSize)
            logging.info('Computed {} operations'.format(count))

main()
