import sys


def solution(file: str) -> int:
    pos_vertical = 0
    pos_horizontal = 0

    with open(file, 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        for _, e in enumerate(rows):
            line = e.split(' ')
            match line[0]:
                case 'forward':
                    pos_horizontal += int(line[1])
                case 'down':
                    pos_vertical += int(line[1])
                case 'up':
                    pos_vertical -= int(line[1])

    return pos_horizontal * pos_vertical


if __name__ == '__main__':
    try:
        print(f"What do you get if you multiply your final horizontal position by your final depth? {solution(sys.argv[1])}")
    except IndexError as e:
        print("No file name to read input from was provided")
    except FileNotFoundError as e:
        print(f"{sys.argv[1]}: {e.args[1]}")
