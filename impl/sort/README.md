# sort

Generate an array of unsorted integers and sort it using the standard library.

## Implementations

* [go](/base/go/sort.go)
* [java](/base/java/src/main/java/compute/sort.java)
* [python](/base/python/sort.py)
* [ruby](/base/ruby/sort.rb)
* [rust](/base/rust/src/sort.rs)

## Example of implementation

```go
// go code

// size of the array to sort
size, _ := strconv.Atoi(os.Getenv("ARRAY_SIZE"))

//create an input array with unsorted values
input := make([]int, size)
incr := 1
for i := 1; i < size; i++ {
    input[i] = -input[i-1] + incr
    incr = -incr
}

// sort the array
sort.Ints(input)
```
