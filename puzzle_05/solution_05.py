import unittest

class TestSolution_05(unittest.TestCase):
    def test_highest_value_from_input_sample(self):
        rows = []

        with open('test_input', 'r') as infile:
            for line in infile.read().splitlines():
                rows.append(line)

            infile.close()

        actual = highest_value_from_input(rows)

        self.assertEqual(actual, 9)

    def test_highest_value_from_input(self):
        rows = []

        with open('input', 'r') as infile:
            for line in infile.read().splitlines():
                rows.append(line)

            infile.close()

        actual = highest_value_from_input(rows)

        self.assertEqual(actual, 990)

    def test_solution_part_1_sample(self):
        actual = solution("test_input", False)

        self.assertEqual(actual, 5)

    def test_solution_part_1(self):
        actual = solution("input", False)

        self.assertEqual(actual, 7269)

    def test_solution_part_2_sample(self):
        actual = solution("test_input", True)

        self.assertEqual(actual, 12)

    def test_solution_part_2(self):
        actual = solution("input", True)

        self.assertEqual(actual, 21140)

def highest_value_from_input(input: list) -> int:
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

    return max(maximums)


def generate_diagram(input: list) -> list:
    # add one, since arrays start at 0
    size = highest_value_from_input(input) + 1

    return [[0 for x in range(size)] for y in range(size)]


def mark_diagram(diagram: list, input: list, part_two: bool) -> list:
    coordinates = []

    for e in input:
        coordinates.append(e.split(' -> '))

    for pos in coordinates:
        # horizontal start
        x_lefthand = int(pos[0].split(',')[0])
        # vertical start
        y_lefthand = int(pos[0].split(',')[1])
        # horizontal end
        x_righthand = int(pos[1].split(',')[0])
        # vertical end
        y_righthand = int(pos[1].split(',')[1])

        end = 0
        start = 0
        step = 0
        
        # mark horizontal lines
        match y_lefthand == y_righthand:
            case True:
                end = x_righthand
                start = x_lefthand

                match x_lefthand < x_righthand:
                    case True:
                        step = 1
                    case False:
                        step = -1

                match step:
                    case -1:
                        for n in range(start, end - 1, step):
                            diagram[y_lefthand][n] = diagram[y_lefthand][n] + 1
                    case 1:
                        for n in range(start, end + 1, step):
                            diagram[y_lefthand][n] = diagram[y_lefthand][n] + 1

                continue

        # mark vertical lines
        match x_lefthand == x_righthand:
            case True:
                end = y_righthand
                start = y_lefthand

                match y_lefthand < y_righthand:
                    case True:
                        step = 1
                    case False:
                        step = -1

                match step:
                    case -1:
                        for n in range(start, end - 1, step):
                            diagram[n][x_lefthand] = diagram[n][x_lefthand] + 1
                    case 1:
                        for n in range(start, end + 1, step):
                            diagram[n][x_lefthand] = diagram[n][x_lefthand] + 1

                continue

        # mark diagonal lines
        match part_two:
            case True:
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
                    diagram[y][x] = diagram[y][x] + 1

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
            match cell:
            # https://peps.python.org/pep-0622/#guards
                case n if cell > 1:
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
