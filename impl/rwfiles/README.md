# rwfiles

Write files and access them randomly.

## Implementations

* [go](/base/go/rwfiles.go)
* [java](/base/java/src/main/java/compute/rwfiles.java)
* [python](/base/python/rwfiles.py)
* [ruby](/base/ruby/rwfiles.rb)
* [rust](/base/rust/src/rwfiles.rs)

## Example of implementation

```ruby
# ruby code

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
```
