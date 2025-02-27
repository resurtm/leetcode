package main

func rearrangeArray(nums []int) []int {
	res, p, n := make([]int, 0, len(nums)), 0, 0
	for len(res) < len(nums) {
		for p < len(nums) && nums[p] < 0 {
			p += 1
		}
		if p < len(nums) && nums[p] > 0 {
			res = append(res, nums[p])
			p += 1
		}
		for n < len(nums) && nums[n] > 0 {
			n += 1
		}
		if n < len(nums) && nums[n] < 0 {
			res = append(res, nums[n])
			n += 1
		}
	}
	return res
}
