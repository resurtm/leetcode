package main

func mergeArrays(nums1 [][]int, nums2 [][]int) [][]int {
	i, j, res := 0, 0, make([][]int, 0)
	for i < len(nums1) || j < len(nums2) {
		if i < len(nums1) && j < len(nums2) {
			if nums1[i][0] < nums2[j][0] {
				res = append(res, nums1[i])
				i += 1
			} else if nums1[i][0] > nums2[j][0] {
				res = append(res, nums2[j])
				j += 1
			} else {
				res = append(res, []int{nums1[i][0], nums1[i][1] + nums2[j][1]})
				i += 1
				j += 1
			}
		} else if i < len(nums1) {
			res = append(res, nums1[i])
			i += 1
		} else if j < len(nums2) {
			res = append(res, nums2[j])
			j += 1
		}
	}
	return res
}
