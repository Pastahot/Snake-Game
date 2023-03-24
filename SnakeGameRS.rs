use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Rect};
use ggez::{Context, GameResult};

const CELL_SIZE: f32 = 20.0;
const WIDTH: f32 = 20.0;
const HEIGHT: f32 = 20.0;
const SNAKE_COLOR: Color = Color::new(0.18, 0.80, 0.44, 1.0);
const FOOD_COLOR: Color = Color::new(0.90, 0.31, 0.23, 1.0);
const BACKGROUND_COLOR: Color = Color::new(0.93, 0.93, 0.93, 1.0);

struct Snake {
    segments: Vec<Rect>,
    direction: Direction,
}

impl Snake {
    fn new(x: f32, y: f32) -> Self {
        let head = Rect::new(x, y, CELL_SIZE, CELL_SIZE);
        let mut segments = Vec::new();
        segments.push(head);
        Self {
            segments,
            direction: Direction::Right,
        }
    }

    fn update(&mut self) {
        let head = self.segments[0];
        let mut new_head = head.clone();
        match self.direction {
            Direction::Up => new_head.y -= CELL_SIZE,
            Direction::Down => new_head.y += CELL_SIZE,
            Direction::Left => new_head.x -= CELL_SIZE,
            Direction::Right => new_head.x += CELL_SIZE,
        }
        self.segments.insert(0, new_head);
        self.segments.pop();
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for segment in &self.segments {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                segment,
                SNAKE_COLOR,
            )?;
            graphics::draw(ctx, &rect, DrawParam::default())?;
        }
        Ok(())
    }

    fn grow(&mut self) {
        let last = self.segments.last().unwrap().clone();
        let mut new_segment = last.clone();
        match self.direction {
            Direction::Up => new_segment.y += CELL_SIZE,
            Direction::Down => new_segment.y -= CELL_SIZE,
            Direction::Left => new_segment.x += CELL_SIZE,
            Direction::Right => new_segment.x -= CELL_SIZE,
        }
        self.segments.push(new_segment);
    }

    fn collides_with_food(&self, food: &Rect) -> bool {
        let head = self.segments[0];
        head.overlaps(food)
    }

    fn collides_with_self(&self) -> bool {
        let head = self.segments[0];
        for segment in self.segments.iter().skip(1) {
            if head.overlaps(segment) {
                return true;
            }
        }
        false
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    position: Rect,
}

impl Food {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Rect::new(x, y, CELL_SIZE, CELL_SIZE),
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), &self.position, FOOD_COLOR)?;
        graphics::draw(ctx, &rect, DrawParam::default())?;
        Ok(())
    }
}

struct GameState {
    snake: Snake,
    food: Food,
    score: i32,
    }
    
    impl GameState {
    fn new() -> Self {
    let snake = Snake::new(WIDTH / 2.0, HEIGHT / 2.0);
    let food = Food::new(CELL_SIZE * 5.0, CELL_SIZE * 5.0);
    Self {
    snake,
    food,
    score: 0,
    }
    }
    fn update(&mut self) {
        self.snake.update();
        if self.snake.collides_with_food(&self.food.position) {
            self.snake.grow();
            self.score += 10;
            self.generate_food();
        }
        if self.snake.collides_with_self() {
            panic!("Game over!");
        }
    }
    
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BACKGROUND_COLOR);
    
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
    
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        graphics::draw(
            ctx,
            &score_text,
            DrawParam::default()
                .dest(graphics::Point2::new(CELL_SIZE, HEIGHT * CELL_SIZE - CELL_SIZE)),
        )?;
        graphics::present(ctx)?;
    
        Ok(())
    }
    
    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, WIDTH as i32) as f32;
        let y = rng.gen_range(0, HEIGHT as i32) as f32;
        self.food = Food::new(x * CELL_SIZE, y * CELL_SIZE);
    }
}
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update();
    self.draw(ctx)?;
    Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => self.snake.direction = Direction::Up,
            KeyCode::Down => self.snake.direction = Direction::Down,
            KeyCode::Left => self.snake.direction = Direction::Left,
            KeyCode::Right => self.snake.direction = Direction::Right,
            _ => (),
        }
    }
}
fn main() -> GameResult<()> {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("snake", "ggez")
    .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(
    (CELL_SIZE * WIDTH) as f32,
    (CELL_SIZE * HEIGHT) as f32,
    ))
    .build()?;
    let mut game = GameState::new();
    event::run(&mut ctx, &mut event_loop, &mut game)
}        
   
