import LineProcessor
import Row

class Row:
    empty = True
    data = []
    target_row = 0

    def calculate_row_content(self, lp: LineProcessor) -> None:
        delta = lp.x_righthand - lp.x_lefthand

        self.data = ['.' for n in range(0, 9)]

        for n in range(0, delta + 1):
            self.data[n] = str(delta + 1)

        self.empty = False

    def __init__(self, lp: LineProcessor) -> None:
        self.calculate_row_content(lp)

