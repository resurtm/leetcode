package main

func getEncryptedString(s string, k int) string {
	res := ""
	for idx := range s {
		res = res + string(s[(idx+k)%len(s)])
	}
	return res
}
