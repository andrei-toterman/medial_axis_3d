#!/usr/bin/python

import sys, random, os

v = sys.argv[1] if len(sys.argv) > 1 else ""
n = int(sys.argv[2]) if len(sys.argv) > 2 else 50
s = int(sys.argv[3]) if len(sys.argv) > 3 else 50
output = open(f"{os.path.basename(__file__).split('.')[0] + (('_' + sys.argv[2]) if len(sys.argv) > 2 else '')}.txt", "w")
for _ in range(n):
    output.write(f"{v} {s * random.random()} {s * random.random()} {s * random.random()}\n")
