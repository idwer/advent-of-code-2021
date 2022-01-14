class LineProcessor:
    x_lefthand = 0
    y_lefthand = 0
    x_righthand = 0
    y_righthand = 0

    def __init__(self, row: list) -> None:
        lefthand = row[0].split(',')
        righthand = row[1].split(',')

        self.x_lefthand = int(lefthand[0])
        self.y_lefthand = int(lefthand[1])
        self.x_righthand = int(righthand[0])
        self.y_righthand = int(righthand[1])
