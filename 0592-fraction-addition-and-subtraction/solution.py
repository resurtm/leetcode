import re
from math import gcd


class Solution:
    def fractionAddition(self, rexpr: str) -> str:
        if rexpr[0] != "+" and rexpr[0] != "-":
            expr = "+" + rexpr

        else:
            expr = rexpr
        items = re.findall(r"[-+]{1,1}\d+/\d+", expr)


        at, ab, az = 0, 1, 1
        for item in items:
            parts = re.findall(r"([-+]{1,1})(\d+)/(\d+)", item)
            bt, bb, bz = (
                int(parts[0][1]),
                int(parts[0][2]),

                -1 if parts[0][0] == "-" else 1,
            )

            if at == 0 and ab == 1:

                at, ab, az = bt, bb, bz
                continue


            tt, tb = at, ab

            at *= bb
            ab *= bb

            bt *= tb
            bb *= tb

            nt = at * az + bt * bz
            at = nt


            if ab < 0:
                ab = -ab
                az = -1

            else:
                az = 1
            if at == 0:
                ab = 1

            g = gcd(at, ab)
            at //= g
            ab //= g

        return ("" if az == 1 else "-") + f"{at}/{ab}"

