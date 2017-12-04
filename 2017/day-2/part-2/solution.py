#!/usr/bin/env python
#
# http://adventofcode.com/2017/day/2
#
# Part Two
#

from sys import stdin

if __name__ == '__main__':
    t = 0
    for l in [[int(y.strip()) for y in x.split('\t')] for x in stdin.readlines()]:
        # This is pretty gross...
        # tldr iterate over all pairs.
        for n in l:
            for m in l:
                # if m divides n, add n // m
                if n is not m and n % m == 0:
                    t += n // m
    print(t)

