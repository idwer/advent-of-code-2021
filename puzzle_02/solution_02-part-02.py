import sys


def solution(filename: str) -> int:
    pos_vertical = 0
    pos_horizontal = 0
    aim = 0

    with open(filename, 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        for row in rows:
            line = row.split(' ')
            match line[0]:
                case 'forward':
                    pos_vertical += aim * int(line[1])
                    pos_horizontal += int(line[1])
                case 'down':
                    aim += int(line[1])
                case 'up':
                    aim -= int(line[1])

    return pos_horizontal * pos_vertical


if __name__ == '__main__':
    print(f'What do you get if you multiply your final horizontal position by your final depth? {solution("test_input")} - using test_input')
    print(f'What do you get if you multiply your final horizontal position by your final depth? {solution("input")}')
