#!/usr/bin/env python
#
# http://adventofcode.com/2017/day/2
#
# Part One
#

from sys import stdin

if __name__ == '__main__':
    t = 0
    # So close to a one-liner...
    # What could have been...
    # It keeps me up at night.
    # ...
    for x in [[int(y.strip()) for y in x.split('\t')] for x in stdin.readlines()]:
        a = min(x)
        b = max(x)
        t += b - a
    print(t)
