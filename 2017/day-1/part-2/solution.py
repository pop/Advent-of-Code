#!/usr/bin/env python
#
# http://adventofcode.com/2017/day/1
#
# Part Two
#
from sys import argv, stdin

def captia(s):
    """
    Computes the sum of a given input string.
    """
    a = b = k = i = 0
    l = list(s)
    n = len(s)
    o = n/2

    for i in range(n):
        a = int(s[i])
        try:
            b = int(s[i+o])
        except IndexError:
            b = int(s[i+o-n])

        if a == b:
            k += a

    return k

if __name__ == '__main__':
    x = stdin.readlines()[0].strip()
    print(captia(x))
