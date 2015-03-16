import socket
import sys

so = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
so.connect(sys.argv[1])

file = so.makefile('rw')
file.write('HELLO\n')
file.flush()
