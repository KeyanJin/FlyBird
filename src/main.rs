use bracket_lib::prelude::*;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 60;
const FRAME_DURATION: f32 = 75.0;
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
    fn render(&self, ctx: &mut BTerm, bird_x: i32) {
        let screen_x = self.x - bird_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, bird: &Bird) -> bool {
        let half_size = self.size / 2;
        let does_x_match = bird.x == self.x;
        let bird_above_y = bird.y + half_size < self.gap_y;
        let bird_below_y = bird.y - half_size > self.gap_y;
        does_x_match && (bird_above_y || bird_below_y)
    }
}
struct Bird {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Bird {
    fn new(x: i32, y: i32) -> Bird {
        Bird {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    fn gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.x += 1;
        self.y += self.velocity as i32;

        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
struct State {
    mode: GameMode,
    bird: Bird,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

enum GameMode {
    Menu,
    Playing,
    Ending,
}
impl State {
    fn new() -> Self {
        State {
            bird: Bird::new(5, 25),
            mode: GameMode::Menu,
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.bird = Bird::new(5, 25);
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Fly_Bird");
        ctx.print_centered(9, "(P) Play a game");
        ctx.print_centered(12, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You're dead!");
        ctx.print_centered(7, &format!("You've earned {} points.", self.score));
        ctx.print_centered(9, "(P) Play a game");
        ctx.print_centered(12, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.bird.gravity();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.bird.flap();
        }
        self.bird.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap!");
        ctx.print(0, 1, format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.bird.x);
        if self.bird.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.bird.x + SCREEN_WIDTH, self.score);
        }
        if self.bird.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.bird) {
            self.mode = GameMode::Ending;
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Ending => self.dead(ctx),
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Fly Bird").build()?;
    main_loop(context, State::new())
}
