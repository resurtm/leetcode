class Solution:
    def compress(self, chars: List[str]) -> int:
        if len(chars) == 1:
            return 1

        i = 0
        cu = chars[0]
        st = 0

        while i < len(chars):
            if chars[i] != cu:
                sz = i - st
                for pp in range(sz - 2):

                    chars.pop(st)
                chars[st] = cu
                if sz > 1:
                    chars[st + 1] = str(sz)
                    i -= sz - 2
                cu = chars[i]
                st = i
            i += 1


        sz = i - st
        for i in range(sz - 2):
            chars.pop(st)
        chars[st] = cu
        if sz > 1:
            chars[st + 1] = str(sz)


        i = 0

        while i < len(chars):
            if len(chars[i]) > 1:
                sz = len(chars[i])
                it = list(chars[i])
                it.reverse()
                chars.pop(i)
                for j in range(sz):
                    chars.insert(i, it[j])
                i += sz

            i += 1

        return len(chars)

