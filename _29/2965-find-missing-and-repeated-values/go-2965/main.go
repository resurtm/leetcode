package main

func findMissingAndRepeatedValues(grid [][]int) []int {
	vis, max := map[int]bool{}, 0
	a, b := -1, -1
	for _, items := range grid {
		for _, item := range items {
			if _, ok := vis[item]; ok {
				a = item
			}
			vis[item] = true
			if item > max {
				max = item
			}
		}
	}
	for item := 1; item <= max; item += 1 {
		if _, ok := vis[item]; !ok {
			b = item
			break
		}
	}
	if b == -1 {
		b = max + 1
	}
	return []int{a, b}
}
