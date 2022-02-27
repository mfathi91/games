from enum import Enum


class Direction(Enum):
    UP = 0
    RIGHT = 1
    DOWN = 2
    LEFT = 3

    def get_opposite_direction(self):
        """ Returns the opposite direction of the current direction.
        """
        if self.value == self.UP.value:
            return self.DOWN
        elif self.value == self.RIGHT.value:
            return self.LEFT
        elif self.value == self.DOWN.value:
            return self.UP
        elif self.value == self.LEFT.value:
            return self.RIGHT

    @classmethod
    def from_value(cls, value: int):
        """ Returns an enum instance for the given direction value.
        Args:
            value (int): the value of the direction.
        """
        if value == 0:
            return cls.UP
        elif value == 1:
            return cls.RIGHT
        elif value == 2:
            return cls.DOWN
        elif value == 3:
            return cls.LEFT
        else:
            raise ValueError(f'Unknown value {value} cannot be converted to a direction')
