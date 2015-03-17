import socket
import sys
import time

so = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
so.connect(sys.argv[1])
file = so.makefile('rw')

time.sleep(0.1)
file.write('HELLO\n')
file.flush()

while True:
    print(file.readline())
