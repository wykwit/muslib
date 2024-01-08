import wave
from sys import argv

from matplotlib import pyplot as plt

with wave.open(argv[1], "rb") as f:
    data = list(f.readframes(f.getnframes()))

l = 100 if len(argv) < 3 else int(argv[2])
plt.title(argv[1])
plt.plot(range(l), data[:l])
plt.show()
