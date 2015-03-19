import socket
import sys
import time
import json

so = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
so.connect(sys.argv[1])
file = so.makefile('rw')

time.sleep(0.1)
data = {}
data["id"] = "arbitrary random"
data["kind"] = "handshake/init"
data["params"] = None
file.write(json.dumps(data) + "\n")
file.flush()

while True:
    print(file.readline())
