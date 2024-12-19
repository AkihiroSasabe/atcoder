
# テストケース
n = 8
k = 78
s = [9,8,5,6,2,9,1,2]

# 愚直な解
for l in range(n):
    temp = 1
    for r in range(l, n):
        temp *= s[r]
        if temp > k:
            print("l = {}, ans = {}, tmep = {}".format(l,  r-l, temp / s[r]))
            break
        if r == n - 1:
            print("l = {}, ans = {}, tmep = {}".format(l,  1+r-l, temp))


# テストケース
# 8 78
# 9 <- 0
# 8 <- 1
# 5 <- 2
# 6 <- 3
# 2 <- 4 *max len = 4*
# 9 <- 5
# 1 <- 6
# 2 <- 7
# 答えは4

# l = 0, ans = 2, tmep = 72.0
# l = 1, ans = 2, tmep = 40.0
# l = 2, ans = 3, tmep = 60.0
# l = 3, ans = 2, tmep = 12.0
# l = 4, ans = 4, tmep = 36
# l = 5, ans = 3, tmep = 18
# l = 6, ans = 2, tmep = 2
# l = 7, ans = 1, tmep = 2