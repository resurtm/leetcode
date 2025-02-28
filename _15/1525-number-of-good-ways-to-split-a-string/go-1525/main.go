package main

func numSplits(s string) int {
	l, r := map[rune]int{}, map[rune]int{}
	lu, ru := map[rune]bool{}, map[rune]bool{}
	for _, ch := range s {
		l[ch] += 1
		lu[ch] = true
	}
	res := 0
	for _, ch := range s {
		l[ch] -= 1
		if l[ch] == 0 {
			delete(lu, ch)
		}
		r[ch] += 1
		ru[ch] = true
		if len(lu) == len(ru) {
			res += 1
		}
	}
	return res
}
