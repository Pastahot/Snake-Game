import React, { useState, useEffect } from 'react';

const ROWS = 20;
const COLS = 20;
const CELL_SIZE = 20;

const SNAKE_INITIAL_STATE = [
  { row: 10, col: 10 },
  { row: 9, col: 10 },
  { row: 8, col: 10 }
];

const KEY_CODES = {
  37: 'left',
  38: 'up',
  39: 'right',
  40: 'down'
};

const getRandomCell = () => ({
  row: Math.floor(Math.random() * ROWS),
  col: Math.floor(Math.random() * COLS)
});

const App = () => {
  const [snake, setSnake] = useState(SNAKE_INITIAL_STATE);
  const [direction, setDirection] = useState('right');
  const [food, setFood] = useState(getRandomCell());
  const [score, setScore] = useState(0);

  useEffect(() => {
    const handleKeyDown = (event) => {
      if (event.keyCode in KEY_CODES) {
        setDirection(KEY_CODES[event.keyCode]);
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, []);

  useEffect(() => {
    const timerId = setInterval(() => {
      const head = snake[0];
      let newHead;
      switch (direction) {
        case 'left':
          newHead = { row: head.row, col: head.col - 1 };
          break;
        case 'up':
          newHead = { row: head.row - 1, col: head.col };
          break;
        case 'right':
          newHead = { row: head.row, col: head.col + 1 };
          break;
        case 'down':
          newHead = { row: head.row + 1, col: head.col };
          break;
        default:
          return;
      }
      if (
        newHead.row < 0 ||
        newHead.row >= ROWS ||
        newHead.col < 0 ||
        newHead.col >= COLS ||
        snake.some((cell) => cell.row === newHead.row && cell.col === newHead.col)
      ) {
        clearInterval(timerId);
      } else {
        const newSnake = [newHead, ...snake];
        if (newHead.row === food.row && newHead.col === food.col) {
          setFood(getRandomCell());
          setScore(score + 1);
        } else {
          newSnake.pop();
        }
        setSnake(newSnake);
      }
    }, 100);
    return () => clearInterval(timerId);
  }, [snake, direction, food, score]);

  const cells = [];
  for (let row = 0; row < ROWS; row++) {
    for (let col = 0; col < COLS; col++) {
      const isSnake = snake.some((cell) => cell.row === row && cell.col === col);
      const isFood = food.row === row && food.col === col;
      cells.push(
        <div
          key={`${row}-${col}`}
          style={{
            width: CELL_SIZE,
            height: CELL_SIZE,
            border: '1px solid gray',
            backgroundColor: isSnake ? 'green' : isFood ? 'red' : 'white'
          }}
        />
      );
    }
  }

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between' }}>
        <div>Score: {score}</div>
        <
