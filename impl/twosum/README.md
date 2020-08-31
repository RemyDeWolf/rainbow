# twosum

Inspired by Leetcode exercise [Two Sum](https://leetcode.com/problems/two-sum/): Given an array of integers, return indices of the two numbers such that they add up to a specific target.

## Implementations

* [go](/base/go/twosum.go)
* [java](/base/java/src/main/java/compute/twosum.java)
* [python](/base/python/twosum.py)
* [ruby](/base/ruby/twosum.rb)
* [rust](/base/rust/src/twosum.rs)

## Example of implementation

```go
// go code
func twoSum(nums []int, target int) []int {
	mapSum := make(map[int]int, 0)
	for i, v := range nums {
		if j, ok := mapSum[v]; ok {
			return []int{j, i}
		}
		mapSum[target-v] = i
	}
	return nil
}
```

## Results

For more information about the testing methodology see [Run the rainbow tests in the cloud](/k8s/README.md).

### Test Output

First run: array of size 100k

```
BATCH_SIZE=20
REPEAT=500
DURATION=10min
MACHINE_TYPE=e2-standard-2 (2xCPU 8GB)
```

| Lang | Workers | Replicas | Count |
| --- | --- | --- | --- |
| go | 1 | 1 | 40900 |
| go | 1 | 2 | 38200 |
| go | 2 | 1 | 52200 |
| java | 1 | 1 | 150840 |
| java | 1 | 2 | 177780 |
| java | 2 | 1 | 153060 |
| python | 1 | 1 | 25780 |
| python | 1 | 2 | 26540 |
| python | 2 | 1 | 26820 |
| ruby | 1 | 1 | 23480 |
| ruby | 1 | 2 | 24160 |
| ruby | 2 | 1 | 24820 |
| rust | 1 | 1 | 50460 |
| rust | 1 | 2 | 55000 |
| rust | 2 | 1 | 56120 |

## rainbow score

| Lang | Score |
| --- | --- |
| java | 100 |
| rust | 32 |
| go | 29 |
| python | 15 |
| ruby | 14 |

Maximum value is 100, see [details](/README.md#rainbow-score) for more info.

python
Start Time:     Sat, 29 Aug 2020 21:45:31 -0700
Completed At:   Sat, 29 Aug 2020 21:45:58 -0700
Duration:       27s

Start Time:     Sat, 29 Aug 2020 21:45:31 -0700
Completed At:   Sat, 29 Aug 2020 21:45:40 -0700
Duration:       9s

Start Time:     Sat, 29 Aug 2020 21:45:31 -0700
Completed At:   Sat, 29 Aug 2020 21:45:35 -0700
Duration:       4s
Pods Statuses:  0 Running / 1 Succeeded / 0 Failed

