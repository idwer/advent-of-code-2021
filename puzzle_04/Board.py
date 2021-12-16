class Board:
    tuples = []
    score = 0

    def parse_list(self, rows: list) -> list:
        t = []

        for row in rows:
            for value in row.split():
                elem = int(value)
                t.append([elem, False])
        return t

    def __init__(self, numbers: list):
        self.tuples = self.parse_list(numbers)
