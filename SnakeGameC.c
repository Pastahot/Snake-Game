#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <conio.h>
#include <Windows.h>

#define WIDTH 20
#define HEIGHT 20
#define CELL_SIZE 2
#define MAX_TAIL 100
#define FPS 10

int x, y, fruit_x, fruit_y, score;
int tail_x[MAX_TAIL], tail_y[MAX_TAIL], tail_length;
bool game_over;
enum direction { STOP, LEFT, RIGHT, UP, DOWN };
enum direction dir;

void setup() {
    srand(time(NULL));
    dir = STOP;
    x = WIDTH / 2;
    y = HEIGHT / 2;
    fruit_x = rand() % WIDTH;
    fruit_y = rand() % HEIGHT;
    score = 0;
    tail_length = 0;
    game_over = false;
}

void draw() {
    system("cls");
    for (int i = 0; i < WIDTH + 2; i++)
        printf("#");
    printf("\n");

    for (int i = 0; i < HEIGHT; i++) {
        for (int j = 0; j < WIDTH; j++) {
            if (j == 0)
                printf("#");
            if (i == y && j == x)
                printf("O");
            else if (i == fruit_y && j == fruit_x)
                printf("F");
            else {
                bool print = false;
                for (int k = 0; k < tail_length; k++) {
                    if (tail_x[k] == j && tail_y[k] == i) {
                        printf("o");
                        print = true;
                    }
                }
                if (!print)
                    printf(" ");
            }

            if (j == WIDTH - 1)
                printf("#");
        }
        printf("\n");
    }

    for (int i = 0; i < WIDTH + 2; i++)
        printf("#");
    printf("\n");
    printf("Score: %d\n", score);
}

void input() {
    if (_kbhit()) {
        switch (_getch()) {
        case 'a':
            dir = LEFT;
            break;
        case 'd':
            dir = RIGHT;
            break;
        case 'w':
            dir = UP;
            break;
        case 's':
            dir = DOWN;
            break;
        case 'x':
            game_over = true;
            break;
        }
    }
}

void logic() {
    int prev_x = tail_x[0];
    int prev_y = tail_y[0];
    int prev2_x, prev2_y;
    tail_x[0] = x;
    tail_y[0] = y;
    for (int i = 1; i < tail_length; i++) {
        prev2_x = tail_x[i];
        prev2_y = tail_y[i];
        tail_x[i] = prev_x;
        tail_y[i] = prev_y;
        prev_x = prev2_x;
        prev_y = prev2_y;
    }
    switch (dir) {
    case LEFT:
        x--;
        break;
    case RIGHT:
        x++;
        break;
    case UP:
        y--;
        break;
    case DOWN:
        y++;
        break;
    default:
        break;
    }

    if (x > WIDTH - 1 || x < 0 || y > HEIGHT - 1 || y < 0)
        game_over = true;

    for (int i = 0; i < tail_length; i++)
        if (tail_x[i] == x && tail_y[i] == y)
            game_over = true;
    if (x == fruit_x && y == fruit_y) {
        srand(time(NULL));
        score += 10;
        fruit_x = rand() % WIDTH;
        fruit_y = rand() % HEIGHT;
        tail_length++;
    }
}

int main() {
    setup();
    while (!game_over) {
        draw();
        input();
        logic();
        Sleep(1000 / FPS);
    }
    printf("Game over!\n");
    printf("Score: %d\n", score);
    return 0;
}
