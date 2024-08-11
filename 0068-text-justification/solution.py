class Solution:
    def fullJustify(self, words: List[str], maxWidth: int) -> List[str]:
        lines = []
        line = ""
        for word in words:
            maybe = (line + " " + word).strip()
            if len(maybe) <= maxWidth:
                line = maybe
            else:
                lines.append(line)
                line = word
        lines.append(line)
        for idx, line in enumerate(lines):
            if idx == len(lines) - 1:
                continue
            rem = maxWidth - len(line)
            sp = line.count(" ")
            if sp == 0:
                continue
            add1 = int(rem / sp)
            add2 = int(rem % sp)
            parts = line.split(" ")
            for pi, pe in enumerate(parts):

                if pi == len(parts) - 1:
                    break
                pe = pe + " " * add1
                if add2 > 0:
                    pe = pe + " "
                    add2 -= 1
                parts[pi] = pe
            lines[idx] = " ".join(parts)
        for idx, line in enumerate(lines):
            lst = maxWidth - len(line)
            lines[idx] += " " * lst
        return lines

