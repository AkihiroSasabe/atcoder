import matplotlib.pyplot as plt

import numpy as np
import math

def f(x):
    c = 1
    d = 100
    y = x + int(math.floor(d // (x+1)))
    # y =  math.floor(d // (x+1))
    return y

def f1(x):
    c = 1
    d = 100
    # y = x + math.floor(d // (x+1))
    y =  int(math.floor(d // (x+1)))
    return y


# x_min = 0
# x_max = 20
x_min = 4
x_max = 11
# x_max = 110



xs = [i for  i in range(x_min, x_max + 1)]
ys = [f(x) for x in xs]
ys0 = [x for x in xs]
ys1 = [f1(x) for x in xs]

plt.plot(xs, ys)
plt.plot(xs, ys0, marker="o")
plt.plot(xs, ys1, marker="x")
plt.savefig("plt_e.png")