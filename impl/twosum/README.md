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
