function romanToInt(s: string): number {
  const nums1: { [key: string]: number } = {
    M: 1000,
    D: 500,
    C: 100,
    L: 50,
    X: 10,
    V: 5,
    I: 1,
  };
  const nums2: { [key: string]: number } = {
    CM: 900,
    CD: 400,
    XC: 90,
    XL: 40,
    IX: 9,
    IV: 4,
  };
  let res = 0;
  let c = s;
  while (c.length > 0) {
    const cs = c.substr(0, 2);
    if (cs in nums2) {
      res += nums2[cs];
      c = c.substr(2);
      continue;
    }
    if (c[0] in nums1) {
      res += nums1[c[0]];
      c = c.substr(1);
      continue;
    }
    throw new Error("invalid scenario");
  }
  return res;
}
