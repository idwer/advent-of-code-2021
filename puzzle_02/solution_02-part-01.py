import sys


def solution(file: str) -> int:
    pos_vertical = 0
    pos_horizontal = 0

    with open(file, 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        for row in rows:
            line = row.split(' ')
            match line[0]:
                case 'forward':
                    pos_horizontal += int(line[1])
                case 'down':
                    pos_vertical += int(line[1])
                case 'up':
                    pos_vertical -= int(line[1])

    return pos_horizontal * pos_vertical


if __name__ == '__main__':
    print(f'What do you get if you multiply your final horizontal position by your final depth? {solution("input")}')
    print(f'What do you get if you multiply your final horizontal position by your final depth? {solution("test_input")} - using test_input')
