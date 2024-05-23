import time

s = time.time()
n = 1_000_000;
a = 0

for i in range(n):
    a += 1 + 1 + 2 + 3 + 4 + 4 / 4 / 3 / 5

t = time.time()
print(t - s)

