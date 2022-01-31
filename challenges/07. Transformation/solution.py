#!/usr/bin/env python

import sys


def encrypt(flag):
    ''.join([chr((ord(flag[i]) << 8) + ord(flag[i + 1])) for i in range(0, len(flag), 2)])


def decrypt(enc):
    return ''.join(chr(c) for c in enc.encode('utf-16-be'))


def main():
    print(decrypt(sys.argv[1]))


if __name__ == '__main__':
    main()
