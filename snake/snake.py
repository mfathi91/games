import os
import random
import time
from typing import Tuple
from kbhit import KBHit
from direction import Direction


class Snake:

    EMPTY = ' '
    BLOCK = '#'
    SNAKE_HEAD = ['^', '>', 'v', '<']
    SNAKE_BODY = '*'
    FOOD = '+'

    def __init__(self, width: int, height: int):
        """
        Creates an instance of this class.

        Args:
            width (int): the width of the space.
            height (int): the height of the space.
        """
        self.width = width
        self.height = height

        # Snake attributes
        x, y = width//4, height//2
        self.snake_coordinates = [(x, y), (x - 1, y), (x - 2, y), (x - 3, y), (x - 4, y), (x - 5, y)]
        self.alive = True
        self.snake_direction = Direction.RIGHT

        # Blocks
        self.block_coordinates = [(i, j) for i in range(self.width) for j in range(self.height)
                                  if i in (0, self.width - 1) or j in (0, self.height - 1)]

        # Score and delay
        self.score = 0
        self.delay = 0.4

        # Food
        self.food_coordinate = self.generate_food_coordinate()

    def is_alive(self):
        """ Returns True if the snake is alive.
        """
        return self.alive

    def get_score(self):
        """ Returns the score of the snake. """
        return self.score

    def get_delay(self):
        """ Returns the delay of the snake's movement.
        """
        return self.delay

    def next_step(self):
        """ Moves the snake to the next step, and sets the corresponding internal status. In case the snake
            should no longer continue its move, 'alive' flag will be set to False.
        """
        x, y = self.snake_coordinates[0][0], self.snake_coordinates[0][1]
        switcher = {
            0: (x, y - 1),
            1: (x + 1, y),
            2: (x, y + 1),
            3: (x - 1, y)
        }
        new_coordinate = switcher[self.snake_direction.value]
        if new_coordinate in self.block_coordinates or new_coordinate in self.snake_coordinates:
            self.alive = False
        elif new_coordinate == self.food_coordinate:
            self.snake_coordinates = [new_coordinate] + self.snake_coordinates
            self.food_coordinate = self.generate_food_coordinate()
            self.score += 1
            self.delay = self.delay - (self.delay * 0.1)
        else:
            self.snake_coordinates = [new_coordinate] + self.snake_coordinates[0:-1]

    def set_direction_if_legal(self, direction: Direction):
        """ Sets the direction of the snake, if it is not equal to the opposite direction of the snake.
        """
        if direction != self.snake_direction.get_opposite_direction():
            self.snake_direction = direction

    def generate_food_coordinate(self) -> Tuple[int, int]:
        """ Generates a random coordinate, taking into account where the blocks are and where the snake is.
        """
        while True:
            x = random.randint(1, self.width - 1)
            y = random.randint(1, self.height - 1)
            food_coordinate = (x, y)
            if food_coordinate not in self.snake_coordinates and food_coordinate not in self.block_coordinates:
                return food_coordinate

    def clear_draw(self):
        """ Clears the terminal and draws the snake's position and its surrounding environment.
        """
        space = ''
        for i in range(self.height):
            row = ''
            for j in range(self.width):
                if (j, i) in self.snake_coordinates:
                    row += self.SNAKE_HEAD[self.snake_direction.value] if (j, i) == self.snake_coordinates[0] \
                        else self.SNAKE_BODY
                elif (j, i) in self.block_coordinates:
                    row += self.BLOCK
                elif (j, i) == self.food_coordinate:
                    row += self.FOOD
                else:
                    row += self.EMPTY
            space += row + '\n'
        os.system('cls' if os.name == 'nt' else 'clear')
        print(space)
        print(f'Score: {self.score}')
        print(f'Speed: {1/self.delay:.1f}')


if __name__ == '__main__':
    kb = KBHit()
    snake = Snake(60, 20)
    while snake.is_alive():
        if kb.kbhit():
            snake.set_direction_if_legal(kb.getarrow())
        snake.next_step()
        snake.clear_draw()
        time.sleep(snake.get_delay())
