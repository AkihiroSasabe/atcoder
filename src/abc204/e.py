# My answer ====
# dist = [0, 1152921504606846976, 1152921504606846976, 105, 86, 176, 112, 26] 26
# Correct answer ====
# dist = [0, 1152921504606846976, 1152921504606846976, 105, 86, 176, 112, 25] 25

# 25 - 12 = 13min
# 13 = t + di/(t+1)
# 13 = t + 54/(t+1)

# 8 7
# 1 8 12 54
# 5 1 70 78
# 6 5 89 89
# 3 2 84 61
# 5 7 26 69
# 1 4 93 44
# 5 4 27 33

xs = []
ys = []

for t in range(0, 30):
    temp = t + 12 + 54 // (t+1)
    print(f"{t=}, {temp=}")
    xs.append(t)
    ys.append(temp)


import matplotlib.pyplot as plt

plt.plot(xs, ys)
plt.savefig("e.png")

t=0, temp=66
t=1, temp=40
t=2, temp=32
t=3, temp=28
t=4, temp=26
t=5, temp=26
t=6, temp=25
t=7, temp=25
t=8, temp=26
t=9, temp=26
t=10, temp=26
t=11, temp=27
t=12, temp=28
t=13, temp=28
t=14, temp=29
t=15, temp=30
t=16, temp=31
t=17, temp=32
t=18, temp=32
t=19, temp=33
t=20, temp=34
t=21, temp=35
t=22, temp=36
t=23, temp=37
t=24, temp=38
t=25, temp=39
t=26, temp=40
t=27, temp=40
t=28, temp=41
t=29, temp=42