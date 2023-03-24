import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

public class SnakeGameJAVA extends JFrame {
    private static final int WIDTH = 400;
    private static final int HEIGHT = 400;
    private static final int CELL_SIZE = 10;
    private int[] snakeX = new int[100];
    private int[] snakeY = new int[100];
    private int snakeLength;
    private int foodX, foodY;
    private String direction;
    private boolean gameOver;
    private Timer timer;

    public SnakeGameJAVA() {
        setTitle("Snake Game");
        setSize(WIDTH, HEIGHT);
        setResizable(false);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setVisible(true);
        addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent e) {
                switch (e.getKeyCode()) {
                    case KeyEvent.VK_UP:
                        if (!direction.equals("down")) direction = "up";
                        break;
                    case KeyEvent.VK_DOWN:
                        if (!direction.equals("up")) direction = "down";
                        break;
                    case KeyEvent.VK_LEFT:
                        if (!direction.equals("right")) direction = "left";
                        break;
                    case KeyEvent.VK_RIGHT:
                        if (!direction.equals("left")) direction = "right";
                        break;
                }
            }
        });
        initGame();
    }

    private void initGame() {
        direction = "right";
        snakeLength = 3;
        snakeX[0] = 50;
        snakeY[0] = 50;
        snakeX[1] = 40;
        snakeY[1] = 50;
        snakeX[2] = 30;
        snakeY[2] = 50;
        generateFood();
        gameOver = false;
        timer = new Timer(100, new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                if (!gameOver) {
                    moveSnake();
                    checkCollision();
                    repaint();
                } else {
                    timer.stop();
                    JOptionPane.showMessageDialog(SnakeGameJAVA.this, "Game Over!");
                }
            }
        });
        timer.start();
    }

    private void generateFood() {
        foodX = (int) (Math.random() * (WIDTH - CELL_SIZE));
        foodY = (int) (Math.random() * (HEIGHT - CELL_SIZE));
        foodX = foodX - foodX % CELL_SIZE;
        foodY = foodY - foodY % CELL_SIZE;
    }

    private void moveSnake() {
        for (int i = snakeLength - 1; i > 0; i--) {
            snakeX[i] = snakeX[i - 1];
            snakeY[i] = snakeY[i - 1];
        }
        switch (direction) {
            case "up":
                snakeY[0] -= CELL_SIZE;
                break;
            case "down":
                snakeY[0] += CELL_SIZE;
                break;
            case "left":
                snakeX[0] -= CELL_SIZE;
                break;
            case "right":
                snakeX[0] += CELL_SIZE;
                break;
        }
    }

    private void checkCollision() {
        if (snakeX[0] < 0 || snakeX[0] >= WIDTH || snakeY[0] < 0 || snakeY[0] >= HEIGHT) {
            gameOver = true;
        }
        for (int i = 1; i < snakeLength; i++) {
            if (snakeX[0] == snakeX[i] && snakeY[0] == snakeY[i]) {
                gameOver = true;
            }
        }
        if (snakeX[0] == foodX && snakeY[0] == foodY) {
            snakeLength++;
            generateFood();
        }
    }
    @Override
    public void paint(Graphics g) {
        super.paint(g);
        for (int i = 0; i < snakeLength; i++) {
            g.setColor(Color.GREEN);
            g.fillRect(snakeX[i], snakeY[i], CELL_SIZE, CELL_SIZE);
        }
        g.setColor(Color.RED);
        g.fillRect(foodX, foodY, CELL_SIZE, CELL_SIZE);
    }

    public static void main(String[] args) {
        new SnakeGameJAVA();
    }
}
