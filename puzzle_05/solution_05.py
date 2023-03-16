def generate_diagram(input: list) -> list:
    # board dimension is the highest value of x1 or x2 or y1 or y2 + 1
    size = 0
    maximums = []

    for pos in input:
        tmp = []

        l = pos.split('->')[0]
        r = pos.split('->')[1]

        tmp.append(l.split(',')[0])
        tmp.append(l.split(',')[1])

        tmp.append(r.split(',')[0])
        tmp.append(r.split(',')[1])

        maximums.append(int(max(tmp)))

    size = max(maximums)

    # add one, since arrays start at 0
    return [['.' for x in range(size + 1)] for y in range(size + 1)]


def mark_diagram(diagram: list, input: list, part_two: bool) -> list:
    coordinates = []

    for e in input:
        coordinates.append(e.split(' -> '))

    for pos in coordinates:
        x_lefthand = int(pos[0].split(',')[0])
        y_lefthand = int(pos[0].split(',')[1])
        x_righthand = int(pos[1].split(',')[0])
        y_righthand = int(pos[1].split(',')[1])

        end = 0
        start = 0
        step = 0
        
        # mark horizontal lines
        if y_lefthand == y_righthand:
            end = x_righthand
            start = x_lefthand

            if x_lefthand < x_righthand:
                step = 1
            else:
                step = -1

            match step:
                case -1:
                    for n in range(start, end - 1, step):
                        if diagram[y_lefthand][n] == '.':
                            diagram[y_lefthand][n] = str(1)
                        else:
                            diagram[y_lefthand][n] = str(int(diagram[y_lefthand][n]) + 1)
                case 1:
                    for n in range(start, end + 1, step):
                        if diagram[y_lefthand][n] == '.':
                            diagram[y_lefthand][n] = str(1)
                        else:
                            diagram[y_lefthand][n] = str(int(diagram[y_lefthand][n]) + 1)

            continue

        # mark vertical lines
        if x_lefthand == x_righthand:
            end = y_righthand
            start = y_lefthand

            if y_lefthand < y_righthand:
                step = 1
            else:
                step = -1

            match step:
                case -1:
                    for n in range(start, end - 1, step):
                        if diagram[n][x_lefthand] == '.':
                            diagram[n][x_lefthand] = str(1)
                        else:
                            diagram[n][x_lefthand] = str(int(diagram[n][x_lefthand]) + 1)
                case 1:
                    for n in range(start, end + 1, step):
                        if diagram[n][x_lefthand] == '.':
                            diagram[n][x_lefthand] = str(1)
                        else:
                            diagram[n][x_lefthand] = str(int(diagram[n][x_lefthand]) + 1)

            continue

        # mark diagonal lines
        if part_two:
            steps = 0
            step_method = 0
            x = x_lefthand
            y = y_lefthand

            if x_lefthand > x_righthand and y_lefthand < y_righthand:
                steps = x_lefthand - x_righthand
                step_method = 1

            if x_lefthand > x_righthand and y_lefthand > y_righthand:
                steps = x_lefthand - x_righthand
                step_method = 2

            if x_lefthand < x_righthand and y_lefthand < y_righthand:
                steps = x_righthand - x_lefthand
                step_method = 3

            if x_lefthand < x_righthand and y_lefthand > y_righthand:
                steps = x_righthand - x_lefthand
                step_method = 4

            for n in range(0, steps + 1):
                if diagram[y][x] == '.':
                    diagram[y][x] = str(1)
                else:
                    diagram[y][x] = str(int(diagram[y][x]) + 1)

                match step_method:
                    case 1:
                        x -= 1
                        y += 1
                    case 2:
                        x -= 1
                        y -= 1
                    case 3:
                        x += 1
                        y += 1
                    case 4:
                        x += 1
                        y -= 1

            continue

    return diagram

def parse_diagram(diagram: list) -> int:
    counter = 0

    for row in diagram:
        for cell in row:
            if cell != '.' and int(cell) > 1:
                counter += 1

    return counter


def solution(filename: str, part_two: bool) -> int:
    result = -1
    rows = []

    with open(filename, 'r') as infile:
    # I really have to get to know how to iterate over a generated object
#         rows = [line for line in infile.read().splitlines()]
        for line in infile.read().splitlines():
            rows.append(line)

        infile.close()

    diagram = generate_diagram(rows)

    if part_two:
        diagram = mark_diagram(diagram, rows, True)
    else:
        diagram = mark_diagram(diagram, rows, False)

    return parse_diagram(diagram)


if __name__ == '__main__':
    print(f"At how many points do at least two lines overlap? {solution('test_input', False)} (should be 5)")
    print(f"At how many points do at least two lines overlap? {solution('test_input', True)} (should be 12)")
    print(f"At how many points do at least two lines overlap? {solution('input', False)} (should be 7269)")
    print(f"At how many points do at least two lines overlap? {solution('input', True)} (should be 21140)")
