import logging
import os
import redis
import concurrent.futures
import compute

def main():
    logging.basicConfig(level=logging.INFO, 
        format='%(asctime)s %(levelname)s %(message)s', 
        datefmt='%Y-%m-%d %H:%M:%S')

    redis_client = redis.Redis(host='redis', port=6379, decode_responses=True)

    base = os.environ['BASE']
    key = '{}-{}.computed'.format(base, os.environ['COMPUTE'])
    logging.info("Redis key: {}".format(key))

    def run_compute(n, batch_size, max_compute):
        count = 0
        while True:
            compute.compute()
            count+=1
            if count%batch_size == 0:
                redis_client.incr(key, batch_size)
                logging.info('[{}] Computed {} operations'.format(n, count))
            if max_compute!=0 and count>=max_compute:
                break

    batch_size = int(os.environ['BATCH_SIZE'])
    workers = int(os.getenv('WORKERS', '1'))
    max_compute = int(os.getenv('MAX_COMPUTE', '0'))
    

    if workers==1:
        run_compute(0, batch_size, max_compute)
    else:
        with concurrent.futures.ThreadPoolExecutor() as executor:
            futures = []
            for n in range(workers):
                logging.info('Starting worker {}'.format(n))
                futures.append(executor.submit(run_compute, n, batch_size, max_compute))
            for future in concurrent.futures.as_completed(futures):
                print(future.result())
    
main()
