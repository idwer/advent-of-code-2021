from LineProcessor import LineProcessor
from Row import Row


def merge_rows(left: list, right: list) -> list:
    # assert len(left) == len(right)

    new_row = []

    for i, e in enumerate(left):
        if e != '.':
            new_row[i] += int(e)

    for i, e in enumerate(right):
        if e != '.':
            new_row[i] += int(e)

    return new_row


def generate_diagram(rows: list) -> list:
    diagram = [Row for n in range(0, len(rows))]

    for i, row in enumerate(rows):
        lp = LineProcessor(row.split(' -> '))
        # print(i, row.split(' -> '))

        # if lp.x_lefthand == lp.x_righthand and diagram[lp.x_lefthand].empty:
        #     diagram[lp.x_lefthand] = Row(lp)
            # continue
        # else:
            # diagram[lp.x_lefthand].data = merge_rows(diagram[lp.x_lefthand].data, Row(lp).data)
            # print(f"placeholder line for merge action - x - row {lp.x_lefthand} - {diagram[lp.x_lefthand].data} {Row(lp).data}")
            # for n, d in enumerate(Row(lp).data):
            #     if d == '.':
            #         continue
            #     print(f"n = {n} d = {d}")
            # print(f"{diagram[lp.x_lefthand].data} {Row(lp).data}")
            #     diagram[lp.x_lefthand].data[n] += int(d)

        if lp.x_lefthand == lp.x_righthand:
            print(i, row.split(' -> '))

            if diagram[lp.x_lefthand].empty:
                diagram[lp.x_lefthand] = Row(lp)
            else:
                print(f"placeholder line for merge action - x - row {lp.x_lefthand} - {diagram[lp.x_lefthand].data} {Row(lp).data}")
                for n, d in enumerate(Row(lp).data):
                    if d == '.':
                        continue
                    t = int(diagram[lp.x_lefthand].data[n])
                    t += int(d)
                    diagram[lp.x_lefthand].data[n] = str(t)

        if lp.y_lefthand == lp.y_righthand:
            print(i, row.split(' -> '))

            if diagram[lp.y_lefthand].empty:
                diagram[lp.y_lefthand] = Row(lp)
            else:
                print(f"placeholder line for merge action - y - row {lp.y_lefthand} - {diagram[lp.y_lefthand].data} {Row(lp).data}")
                for n, d in enumerate(Row(lp).data):
                    if d == '.':
                        continue
                    t = int(diagram[lp.y_lefthand].data[n])
                    t += int(d)
                    diagram[lp.y_lefthand].data[n] = str(t)

    return diagram


def solution(filename: str, foo: bool) -> int:
    with open(filename, 'r') as infile:
        rows = [line for line in infile.read().splitlines()]
        infile.close()

    result = 0

    diagram = generate_diagram(rows)

    for i, row in enumerate(diagram):
        print(i, row.data, row.empty)

    return result


if __name__ == '__main__':
    print(f"At how many points do at least two lines overlap? {solution('test_input', False)} (should be 5)")
    # print(f"At how many points do at least two lines overlap? {solution('input', False)} (should be xxx)")
