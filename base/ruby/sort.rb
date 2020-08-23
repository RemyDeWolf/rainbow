
def compute()
    # size of the array to sort
    size = ENV['ARRAY_SIZE'].to_i

    # create an input array with unsorted values
    arr = Array.new(size, 0)
    incr = 1
    (1..size).each do |i|
        arr[i] = -arr[i-1] + incr
        incr = -incr
    end
    
    arr.sort
end
