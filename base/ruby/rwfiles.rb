
def compute()
    writes = ENV['WRITES'].to_i
    reads = ENV['READS'].to_i
    file_size = ENV['FILE_SIZE'].to_i

    # create the file data
    data = "0" * file_size

    # write the files
    files = Array.new(writes)
    writes.times { |i|
        path = "/data/#{i}"
        File.write(path, data)
    }

    # read the files
    reads.times {
        index = rand(writes)
        path = "/data/#{index}"
        file = File.open(path)
        file_data = file.read
        file.close
    }
end
