require 'securerandom'

def compute()
    size = ENV['HASHTABLE_SIZE'].to_i
    read_count = ENV['READ_COUNT'].to_i

    # create an input map and an array
    hashtable = Hash.new
    keys = Array.new(size, 0)

    # init the map
    size.times { |i|
        key = SecureRandom.uuid
        hashtable[key] = key
        keys[i] = key
    }

    # access map
    read_count.times {
        key = keys[rand(size)]
        hashtable[key]
    }
end
