import socket

so = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
so.connect('/tmp/cylonn')
