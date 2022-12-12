import numpy as np
import matplotlib.pyplot as plt

a, b =\
1000000000000000000, 100

#  8_772_053_214_538
# 20_000_000_000_000

# x = [i for i in range(1, 18_000_000_000_000, 10_000_000)]
# x = [i for i in range(1, 10_000_000_000_000, 1_000_000_000_000)]
x = [i for i in range(1, 100_000_000_000, 100_000_000)]
y = []
for i in x:
    time = a / np.sqrt(1+i) + i*b
    # time = np.log10(a / np.sqrt(1+i) + i*b)
    y.append(time)

plt.ylim(0.0,30_000_000_000_000)
plt.plot(x, y)

plt.savefig("graph_e.png")