package main

func countMatchingSubarrays(nums []int, pattern []int) int {
	type entry struct {
		n int
		p int
	}
	res, acc := 0, []entry{}
	check := func(acc []entry) []entry {
		acc_next := make([]entry, 0, len(acc))
		for _, it := range acc {
			if it.p == len(pattern) {
				res += 1
				continue
			}
			if pattern[it.p] == 1 && nums[it.n+1] > nums[it.n] ||
				pattern[it.p] == 0 && nums[it.n+1] == nums[it.n] ||
				pattern[it.p] == -1 && nums[it.n+1] < nums[it.n] {
				acc_next = append(acc_next, entry{n: it.n + 1, p: it.p + 1})
			}
		}
		return acc_next
	}
	for idx := 0; idx < len(nums); idx += 1 {
		if idx < len(nums)-len(pattern) {
			acc = append(acc, entry{n: idx, p: 0})
		}
		acc = check(acc)
	}
	return res
}
