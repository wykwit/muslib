from sys import argv
from matplotlib import pyplot as plt

with open(argv[1], "r") as f:
    data = [int(x) for x in f.read().strip().split()]

l = 100 if len(argv) < 3 else int(argv[2])
plt.plot(range(l), data[:l])
plt.show()
