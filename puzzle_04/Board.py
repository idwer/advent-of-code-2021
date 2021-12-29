class Board:
    tuples = []
    won = False

    def get_sum_of_unmarked_numbers(self) -> int:
        ret = 0

        for elem in self.tuples:
            if not elem[1]:
                ret += elem[0]

        return ret

    def mark_number(self, number: int) -> None:
        for elem in self.tuples:
            if elem[0] is number:
                elem[1] = True

    def has_winning_row_or_column(self, horizontal: bool) -> bool:
        if horizontal:
            for n in range(0, 5 ** 2, 5):
                if self.tuples[n + 0][1] and \
                        self.tuples[n + 1][1] and \
                        self.tuples[n + 2][1] and \
                        self.tuples[n + 3][1] and \
                        self.tuples[n + 4][1]:
                    self.won = True
                    return True
        else:
            for n in range(0, 5):
                if self.tuples[n + 0 * 5][1] and \
                        self.tuples[n + 1 * 5][1] and \
                        self.tuples[n + 2 * 5][1] and \
                        self.tuples[n + 3 * 5][1] and \
                        self.tuples[n + 4 * 5][1]:
                    self.won = True
                    return True
        return False

    def parse_list(self, rows: list) -> list:
        items = []

        for row in rows:
            for value in row.split():
                elem = int(value)
                items.append([elem, False])
        return items

    def __init__(self, numbers: list) -> None:
        self.tuples = self.parse_list(numbers)
