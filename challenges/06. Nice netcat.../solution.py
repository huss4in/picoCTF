#!/usr/bin/env python

import sys
import socket


class Netcat:

    def __init__(self, host: str, port: int) -> None:
        """Initialize the class."""
        self.host = str(host)
        self.port = int(port)

    def get(self):
        """Connect to host and retrieve the bytes."""

        print(f'Connecting to {self.host}:{self.port}...')

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.host, int(self.port)))
            print('{}:{}'.format(*s.getpeername()))
            s.shutdown(socket.SHUT_WR)

            print('\nReceiving bytes...')
            self.data = s.recv(1024)

            print(self.data)

        return self

    def decode(self) -> str:
        """Decode the bytes."""

        print("\nDecoding bytes...")

        self.flag = ''.join(chr(int(b.decode())) for b in self.data.split(str.encode('\n'))[:-2])

        return self

    def print(self) -> None:
        """Print the flag."""
        print(f'{self.flag}')


def main():
    Netcat(sys.argv[1], sys.argv[2]).get().decode().print()


if __name__ == '__main__':
    main()
