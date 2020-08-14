require 'logger'
require 'redis'

require_relative 'compute'  

logger = Logger.new(STDOUT)

base = ENV["BASE"]
name = ENV['COMPUTE']
key = "#{base}-#{name}.computed"
logger.info("Redis key: #{key}")

redis = Redis.new(host: "redis", :reconnect_attempts => 10, :reconnect_delay => 1.5)

batchSize = ENV['BATCH_SIZE'].to_i
count = 0

while true  do
    compute()
    count=count+1
    if (count%batchSize == 0) then
        redis.incrby(key, batchSize)
        logger.info("Computed #{count} operations")
    end
end
