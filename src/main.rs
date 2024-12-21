// 导入 `bracket_lib` 库的前置内容，假设这个库提供了游戏开发相关的功能
use bracket_lib::prelude::*;

// 屏幕宽度常量，单位为像素（假设），用于定义游戏窗口的横向尺寸
const SCREEN_WIDTH: i32 = 80;
// 屏幕高度常量，单位为像素（假设），用于定义游戏窗口的纵向尺寸
const SCREEN_HEIGHT: i32 = 60;
// 每帧持续时间，单位为毫秒（假设），用于控制游戏的帧率相关逻辑
const FRAME_DURATION: f32 = 75.0;

// `Obstacle` 结构体表示游戏中的障碍物
// 包含其在屏幕上的 x 坐标位置、上下两部分之间的间隙 y 坐标以及障碍物的大小（高度相关）
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

// `Obstacle` 结构体的方法实现
impl Obstacle {
    // 创建一个新的障碍物实例
    // `x` 参数指定初始的 x 坐标位置，`score` 参数可能用于根据游戏得分情况调整障碍物属性（如大小）
    fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    // 在给定的渲染上下文 `ctx` 中渲染障碍物，需要传入小鸟的 x 坐标用于相对定位
    fn render(&self, ctx: &mut BTerm, bird_x: i32) {
        let screen_x = self.x - bird_x;
        let half_size = self.size / 2;
        // 渲染障碍物上半部分
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // 渲染障碍物下半部分
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    // 判断小鸟是否与该障碍物发生碰撞
    fn hit_obstacle(&self, bird: &Bird) -> bool {
        let half_size = self.size / 2;
        let does_x_match = bird.x == self.x;
        let bird_above_y = bird.y + half_size < self.gap_y;
        let bird_below_y = bird.y - half_size > self.gap_y;
        does_x_match && (bird_above_y || bird_below_y)
    }
}

// `Bird` 结构体表示游戏中的小鸟，包含其在屏幕上的 x、y 坐标以及垂直方向的速度
struct Bird {
    x: i32,
    y: i32,
    velocity: f32,
}

// `Bird` 结构体的方法实现
impl Bird {
    // 创建一个新的小鸟实例，指定初始的 x 和 y 坐标位置
    fn new(x: i32, y: i32) -> Bird {
        Bird {
            x,
            y,
            velocity: 0.0,
        }
    }

    // 在给定的渲染上下文 `ctx` 中渲染小鸟（可能只是绘制一个代表小鸟的图形符号等）
    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // 模拟小鸟受到重力影响的逻辑，更新小鸟的速度、坐标位置等
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

    // 模拟小鸟扇动翅膀（跳跃等操作）的逻辑，改变小鸟的垂直速度
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

// 游戏状态的枚举类型，包含菜单、游戏进行中、游戏结束三种状态
enum GameMode {
    Menu,
    Playing,
    Ending,
}

// `State` 结构体用于管理整个游戏的状态，包含当前游戏模式、小鸟实例、帧时间、障碍物实例以及游戏得分等信息
struct State {
    mode: GameMode,
    bird: Bird,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

// `State` 结构体的方法实现
impl State {
    // 创建一个新的游戏状态实例，初始化各个属性为默认值（如初始位置、初始得分等）
    fn new() -> Self {
        State {
            bird: Bird::new(5, 25),
            mode: GameMode::Menu,
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    // 重置游戏状态，将游戏模式设置为 `Playing`，并重新初始化小鸟、障碍物和得分等属性
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.bird = Bird::new(5, 25);
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    // 处理游戏主菜单界面的逻辑，显示菜单选项，根据用户输入执行相应操作（如开始游戏或退出）
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

    // 处理游戏结束界面的逻辑，显示游戏结束相关信息（如得分），并根据用户输入决定是否重新开始或退出
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

    // 处理游戏进行中的核心逻辑，包括更新帧时间、处理小鸟的行为（重力、跳跃等）、渲染元素、碰撞检测以及得分更新等
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

// 实现 `GameState` trait（假设来自 `bracket_lib` 库，用于让 `State` 结构体符合游戏状态更新的规范）
impl GameState for State {
    // 每帧调用的更新函数，根据当前游戏模式调用相应的处理函数（菜单、游戏进行中、游戏结束）
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Ending => self.dead(ctx),
        }
    }
}

// 游戏的主函数入口，创建游戏窗口上下文，并进入游戏主循环，传入初始的游戏状态
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Fly Bird").build()?;
    main_loop(context, State::new())
}