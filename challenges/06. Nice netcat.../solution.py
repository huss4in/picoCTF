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

        flag: str = self.decode(data)

        print(os.environ.get('FLAG_STYLE')
              .replace('\\n', '\n')
              .replace('\\e', '\033')
              .replace('%s', flag))

    def nc(self) -> bytes:
        """Connect to host and receive bytes."""

        print("\n{}{}...".format(
            colored("Connecting to ", 'blue', attrs=['bold']),
            colored(f"{self.host}:{self.port}", 'yellow', attrs=['bold']),
        ))

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.host, int(self.port)))
            print('{}:{}'.format(*s.getpeername()))
            s.shutdown(socket.SHUT_WR)

            print("\n{}...".format(
                colored("Receiving bytes", 'blue', attrs=['bold']),
            ))

            data = s.recv(1024)

            print(data)

            return data

    def decode(self, data) -> str:
        """Decode the bytes."""

        print("\n{}...".format(
            colored("Decoding bytes", 'blue', attrs=['bold']),
        ))

        flag = ''.join(chr(int(b.decode())) for b in data.split(str.encode('\n'))[:-2])

        return flag


if __name__ == '__main__':
    main()
