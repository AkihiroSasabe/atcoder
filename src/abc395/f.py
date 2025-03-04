import numpy as np
import cv2

n,x = list(map(int, input().split()))

u = []
d = []
max_sum = 0
min_sum = []
all_sum = 0
for i in range(n):
    ui,di = list(map(int, input().split()))
    u.append(ui)
    d.append(di)
    all_sum = all_sum+ui+di
    max_sum = max(max_sum, ui+di)
    min_sum.append((ui+di, i))

min_sum.sort()
print(min_sum)

bin = max_sum // n
a = np.zeros((max_sum+1, max_sum+1,3), np.uint8)
for i in range(n):
    ui = u[i]
    di = d[i]
    a[0:ui+1, i*bin:(i+1)*bin, 2] = 255
    a[max_sum+1-di:max_sum+1, i*bin:(i+1)*bin, 1] = 255
    cv2.imwrite("f.png", a)

ans = 9460
h = (all_sum - ans) / n
print(h)