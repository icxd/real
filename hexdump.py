import sys

# Python 3.10

def hexdump(data, width=16, sep=' ', offset=0):
    """Return a hexdump of data as a string."""
    lines = []
    for i in range(0, len(data), width):
        chunk = data[i:i+width]
        hexa = sep.join(f'{b:02x}' for b in chunk)
        text = ''.join(chr(b) if 0x20 <= b < 0x7f else '.' for b in chunk)
        lines.append(f'{i+offset:08x}  {hexa:<48}  {text}')
    return '\n'.join(lines)

if __name__ == '__main__':
    with open(sys.argv[1], 'rb') as f:
        data = f.read()
    print(hexdump(data))