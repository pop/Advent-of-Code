#!/usr/bin/env python
#
# http://adventofcode.com/2017/day/1
#
# Part One
#
from sys import argv, stdin

def circular(l):
    """ This generator yeilds each element of a given list.
    When it reaches the end of the list it wraps around.
    This continues forever and ever and ever and ever...
    """
    i = 0
    while True:
        if i == len(l):
            i = 0
        yield l[i] 
        i += 1

def captia(s):
    """
    Computes the sum of a given input string.
    """
    a = b = k = i = 0

    for x in circular(list(s)):
        a = int(x)

        if a == b:
            k += a

        if i == len(s):
            break

        i += 1
        b = a

    return k

if __name__ == '__main__':
    x = stdin.readlines()[0].strip()
    print(captia(x))
