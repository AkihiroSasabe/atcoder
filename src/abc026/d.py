import matplotlib.pyplot as plt
import math

# 入力例1
# a,b,c=1,1,1
# xs = [i / 10000 for i in range(2000000)]
# xmin=97
# xmax=103
# ymin=90
# ymax=110

# 入力例2
a,b,c=53,82, 49
xs = [i / 100 for i in range(300)]
xmin=1.5
xmax=2.0
ymin=0
ymax=210

ys = [a*x+b*math.sin(c*x*math.pi) for x in xs]
ys2 = [a*x for x in xs]
ys3 = [100 for x in xs]

plt.plot(xs,ys)
plt.plot(xs,ys2)
plt.plot(xs,ys3)
plt.xlim(xmin, xmax)
plt.ylim(ymin, ymax)
plt.savefig("d.png")