require 'ruby2d'

# Set up the window
set title: 'Snake Game', width: 640, height: 480

# Set up the snake
snake = [
  { x: 10, y: 10 },
  { x: 9, y: 10 },
  { x: 8, y: 10 }
]

# Set up the initial direction
direction = :right

# Set up the food
food = {
  x: rand(get(:width) / 10) * 10,
  y: rand(get(:height) / 10) * 10
}

# Set up the score
score = 0

# Set up the game loop
update do
  # Move the snake in the current direction
  case direction
  when :up
    snake.unshift({ x: snake.first[:x], y: snake.first[:y] - 10 })
  when :down
    snake.unshift({ x: snake.first[:x], y: snake.first[:y] + 10 })
  when :left
    snake.unshift({ x: snake.first[:x] - 10, y: snake.first[:y] })
  when :right
    snake.unshift({ x: snake.first[:x] + 10, y: snake.first[:y] })
  end

  # Check if the snake has collided with the walls
  if snake.first[:x] < 0 || snake.first[:x] > get(:width) - 10 || snake.first[:y] < 0 || snake.first[:y] > get(:height) - 10
    close
  end

  # Check if the snake has collided with itself
  if snake[1..-1].any? { |segment| segment[:x] == snake.first[:x] && segment[:y] == snake.first[:y] }
    close
  end

  # Check if the snake has eaten the food
  if snake.first[:x] == food[:x] && snake.first[:y] == food[:y]
    food = {
      x: rand(get(:width) / 10) * 10,
      y: rand(get(:height) / 10) * 10
    }
    score += 1
  else
    snake.pop
  end

  # Clear the screen
  clear

  # Draw the food
  Square.new(x: food[:x], y: food[:y], size: 10, color: 'red')

  # Draw the snake
  snake.each do |segment|
    Square.new(x: segment[:x], y: segment[:y], size: 10, color: 'green')
  end

  # Draw the score
  Text.new("Score: #{score}", x: 10, y: 10, size: 20, color: 'white')
end

# Handle keyboard input
on :key_down do |event|
  case event.key
  when 'up'
    if direction != :down
      direction = :up
    end
  when 'down'
    if direction != :up
      direction = :down
    end
  when 'left'
    if direction != :right
      direction = :left
    end
  when 'right'
    if direction != :left
      direction = :right
    end
  end
end

# Show the window
show
