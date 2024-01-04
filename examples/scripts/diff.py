from sys import argv

with open(argv[1], "r") as f:
    d1 = [int(x) for x in f.read().strip().split()]

with open(argv[2], "r") as f:
    d2 = [int(x) for x in f.read().strip().split()]

if len(d1) != len(d2):
    exit(1)

for i in range(len(d1)):
    if d1[i] != d2[i]:
        print(i, d1[i], d2[i])

