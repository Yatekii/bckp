import sys
from functools import partial

chunks = [[0, 0]]
last_size = 1

with open(sys.argv[1], 'rb') as f:
    i = 0
    num = 0
    for block in iter(partial(f.read, 2**13), b''):
        s = 0
        i += 1
        num = 0
        for b in block:
            s += b
            num += 1
        s %= 2**12
        if s == 0:
            chunks[-1][1] = i
            chunks.append([i, 0])
            chunk = chunks[-2]
            print(chunk[1] * (2 ** 13))
    chunks[-1][1] = i
    print((chunks[-1][1] - 1) * (2 ** 13) + num)
