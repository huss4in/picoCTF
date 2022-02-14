#!/usr/bin/env python

import os
import socket

from termcolor import colored


def main():
    Netcat('mercury.picoctf.net', 7449).flag()


class Netcat:

    def __init__(self, host: str, port: int) -> None:
        """Initialize."""
        self.host = host
        self.port = port

    def flag(self) -> str:
        """Print the flag."""

        data: bytes = self.nc()

        flag_bytes: list[bytes] = self.parse(data)

        flag_chars: tuple[str] = self.decode(flag_bytes)

        flag: str = self.join(flag_chars)

        print(os.environ.get('FLAG_STYLE')
              .replace('\\n', '\n')
              .replace('\\e', '\033')
              .replace('%s', flag), end='')

    def nc(self) -> bytes:
        """Connect to host and receive bytes."""

        print("\n{}, {}".format(
            colored("Make TCP connection", 'blue', attrs=['bold']),
            colored(f"{self.host}:{self.port}", 'yellow'),
        ))

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.host, int(self.port)))
            print('{}:{}'.format(*s.getpeername()))
            s.shutdown(socket.SHUT_WR)

            print("\n{}".format(
                colored("Receive bytes", 'blue', attrs=['bold']),
            ))

            data = s.recv(1024)

            print(data)

            return data

    def parse(self, data: bytes) -> list[bytes]:
        """Parse the bytes."""

        print("\n{}".format(
            colored("Parse", 'blue', attrs=['bold']),
        ))

        flag_bytes = data.split(b' \n')[:-1]

        print(flag_bytes)

        return flag_bytes

    def decode(self, flag_bytes: list[bytes]) -> tuple[str]:
        """Decode the bytes."""

        print("\n{}, {}".format(
            colored("Decode", 'blue', attrs=['bold']),
            colored("ASCII", 'yellow')
        ))

        flag_chars = tuple(chr(int(byte)) for byte in flag_bytes)

        print(flag_chars)

        return flag_chars

    def join(self, flag_chars: tuple[str]) -> str:
        """Join the characters."""

        print("\n{}".format(
            colored("Join", 'blue', attrs=['bold']),
        ))

        flag = ''.join(flag_chars)[:-1]

        return flag


if __name__ == '__main__':
    main()
