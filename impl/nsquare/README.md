# nsquare

Generate an array of size n and compute a n square operation on it.

## Implementations

* [go](/base/go/nsquare.go)
* [java](/base/java/src/main/java/compute/nsquare.java)
* [python](/base/python/nsquare.py)
* [ruby](/base/ruby/nsquare.rb)
* [rust](/base/rust/src/nsquare.rs)

## Example of implementation

```python
def compute():
    size = int(os.environ['NSQUARE_ARRAY_SIZE'])
    input = [0] * size

    # n square read/write operation
    for i in range(size):
        for j in range(size):
            input[j]=(input[i] + j) / 2
```
