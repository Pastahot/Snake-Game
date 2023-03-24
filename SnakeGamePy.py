import random
import time

WIDTH = 20
HEIGHT = 20

SNAKE_HEAD = 'O'
SNAKE_BODY = '*'
FOOD = 'X'
EMPTY = ' '

def initialize_board():
    board = [[EMPTY for _ in range(WIDTH)] for _ in range(HEIGHT)]
    return board

def print_board(board, score):
    print("Score:", score)
    print("+--" * WIDTH + "+")
    for row in board:
        print("|", end="")
        for cell in row:
            print(cell, end=" ")
        print("|")
    print("+--" * WIDTH + "+")

def generate_food(snake, board):
    while True:
        x = random.randint(0, WIDTH - 1)
        y = random.randint(0, HEIGHT - 1)
        if board[y][x] == EMPTY and (x, y) not in snake:
            board[y][x] = FOOD
            return (x, y)

def get_input():
    while True:
        key = input("Enter direction (WASD): ").lower()
        if key in ['w', 'a', 's', 'd']:
            return key

def move_snake(snake, direction):
    dx = -1 if direction == 'a' else 1 if direction == 'd' else 0
    dy = -1 if direction == 'w' else 1 if direction == 's' else 0
    head = (snake[0][0] + dx, snake[0][1] + dy)
    if head in snake or head[0] < 0 or head[0] >= WIDTH or head[1] < 0 or head[1] >= HEIGHT:
        return None
    snake.insert(0, head)
    return head

def play_game():
    board = initialize_board()
    snake = [(WIDTH//2, HEIGHT//2)]
    food = generate_food(snake, board)
    score = 0
    direction = 'd'

    while True:
        print_board(board, score)
        key = get_input()
        if key in ['w', 'a', 's', 'd']:
            direction = key

        head = move_snake(snake, direction)
        if head is None:
            print("Game over!")
            break

        if head == food:
            food = generate_food(snake, board)
            score += 1
        else:
            x, y = snake.pop()
            board[y][x] = EMPTY

        board[head[1]][head[0]] = SNAKE_HEAD
        for x, y in snake[1:]:
            board[y][x] = SNAKE_BODY

        time.sleep(0.1)

if __name__ == '__main__':
    play_game()
