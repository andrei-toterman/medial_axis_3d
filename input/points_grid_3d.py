#!/usr/bin/python

import sys, os

v = sys.argv[1] if len(sys.argv) > 1 else ""
n = int(sys.argv[2]) if len(sys.argv) > 2 else 3
s = int(sys.argv[3]) if len(sys.argv) > 3 else 10
output = open(f"{os.path.basename(__file__).split('.')[0] + (('_' + sys.argv[2]) if len(sys.argv) > 2 else '')}.txt", "w")
for i in range(n):
    for j in range(n):
        for k in range(n):
            output.write(f"{v} {i * s} {j * s} {k * s}\n")
