import dis
import marshal


byte_file = 'main.pyc'

with open(byte_file, 'rb') as f:
    meta = f.read(8)
    code = marshal.load(f)
    print(dis.dis(meta))
    print(dis.dis(code))
