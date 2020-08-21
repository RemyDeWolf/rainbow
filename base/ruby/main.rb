require 'logger'
require 'redis'
require 'concurrent'

require_relative 'compute'  

$logger = Logger.new(STDOUT)

base = ENV["BASE"]
name = ENV['COMPUTE']
$key = "#{base}-#{name}.computed"
$logger.info("Redis key: #{$key}")

$redis = Redis.new(host: "redis", :reconnect_attempts => 10, :reconnect_delay => 1.5)

$batchSize = ENV['BATCH_SIZE'].to_i
workers = (ENV['WORKERS'] || '1').to_i

def run_compute(n)
    $logger.info("Starting worker #{n}")
    count = 0
    while true  do
        compute()
        count=count+1
        if (count % $batchSize == 0) then
            $redis.incrby($key, $batchSize)
            $logger.info("[#{n}] Computed #{count} operations")
        end
    end
end

if workers<=1
    run_compute(0)
else
    pool = Concurrent::FixedThreadPool.new(workers)
    workers.times do |i|
        pool.post do
            run_compute(i)
        end
    end
    pool.shutdown
    pool.wait_for_termination
end
