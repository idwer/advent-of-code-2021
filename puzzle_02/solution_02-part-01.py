import sys


def do(file):
    pos_vertical = 0
    pos_horizontal = 0

    with open(file) as infile:
        rows = [n for n in infile.read().splitlines()]

        for _, e in enumerate(rows):
            line = e.split(' ')
            if 'forward' in line[0]:
                pos_horizontal += int(line[1])
            if 'down' in line[0]:
                pos_vertical += int(line[1])
            if 'up' in line[0]:
                pos_vertical -= int(line[1])

    print(f"What do you get if you multiply your final horizontal position by your final depth? {pos_horizontal * pos_vertical}")


if __name__ == '__main__':
    do(sys.argv[1])
