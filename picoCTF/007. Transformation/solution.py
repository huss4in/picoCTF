#!/usr/bin/env python

import os
import sys

from termcolor import colored


def main():
    Transformation('enc').flag()


class Transformation:

    def __init__(self, filename: str) -> None:
        """Initialize."""
        self.filename = filename

    def flag(self) -> None:

        encrypted: str = self.read()

        flag: str = self.decrypt(encrypted)

        print(os.environ.get('FLAG_STYLE')
              .replace('\\n', '\n')
              .replace('\\e', '\033')
              .replace('%s', flag), end='')

    def read(self) -> str:
        """Read the encrypted flag."""

        print("\n{}, {}".format(
            colored("Read enctyped flag", 'blue', attrs=['bold']),
            colored("'enc'", 'yellow'),
        ))

        try:
            with open(self.filename, 'r') as f:
                encrypted = f.read()
                print(encrypted)
                return encrypted
        except Exception:
            sys.stderr.write(
                colored("\nCouldn't read file!\n", 'red', attrs=['bold', 'blink'])
            )
            exit(1)

    def encrypt(self, flag):
        """The function the encrypted the flag."""
        ''.join([chr((ord(flag[i]) << 8) + ord(flag[i + 1])) for i in range(0, len(flag), 2)])

    def decrypt(self, enc):
        """Decrypt the encrypted flag."""

        print("\n{}, {}".format(
            colored("Decode", 'blue', attrs=['bold']),
            colored("utf-16-be", 'yellow'),
        ))

        return ''.join(chr(c) for c in enc.encode('utf-16-be'))


if __name__ == '__main__':
    main()
