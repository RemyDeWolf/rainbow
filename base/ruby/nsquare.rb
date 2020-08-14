
def compute()
    size = ENV['NSQUARE_ARRAY_SIZE'].to_i
    input = Array.new(size, 0)

    size.times { |i| 
        size.times { |j| 
            input[j]=(input[i] + j) / 2
        }
    }
end
