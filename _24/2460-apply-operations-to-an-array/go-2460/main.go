package main

func applyOperations(nums []int) []int {
	res1, res2 := make([]int, 0, len(nums)), make([]int, 0, len(nums))
	for i := range nums {
		if i != len(nums)-1 && nums[i] == nums[i+1] {
			nums[i] *= 2
			nums[i+1] = 0
		}
		if nums[i] == 0 {
			res2 = append(res2, nums[i])
		} else {
			res1 = append(res1, nums[i])
		}
	}
	return append(res1, res2...)
}
