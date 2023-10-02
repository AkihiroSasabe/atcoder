
import random

"""
以下のテストケースを作るコードである
入力
N K P
C1 A11 A12 ... A1K
C2 A21 A22 ... A2K
...
CN AN1 AN2 ... ANK

入力例
4 3 5
5 3 0 2
3 1 2 3
3 2 4 0
1 0 1 4
"""

n = random.randrange(1, 101)
k = random.randrange(1, 6)
p = random.randrange(1, 6)

cs = [random.randrange(1, 1_000_000_001) for i in range(n)]
a = [[random.randrange(0, p+1) for j in range(k)] for i in range(n)]

# n = 100
# k = 5
# p = 5

# cs = [1_000_000_000 for i in range(n)]
# a = [[1, 1, 1, 1, 1] for i in range(n)]

print(n, k, p)
for i in range(n):
    print("{}".format(cs[i]), end=' ')
    for j in range(len(a[i])):
        print("{}".format(a[i][j]), end=' ')
    print("")