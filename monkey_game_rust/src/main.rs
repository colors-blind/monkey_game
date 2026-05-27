use macroquad::prelude::*;
use ::rand::Rng;

// --- 常量 ---
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const PLATFORM_HEIGHT: f32 = 200.0;
const PLATFORM_MIN_WIDTH: i32 = 40;
const PLATFORM_MAX_WIDTH: i32 = 120;
const GAP_MIN: i32 = 60;
const GAP_MAX: i32 = 300;

const MONKEY_SIZE: f32 = 40.0;
const MONKEY_SPEED: f32 = 300.0;

const STAFF_GROW_SPEED: f32 = 200.0;
const STAFF_WIDTH: f32 = 4.0;
const STAFF_ROTATE_SPEED: f32 = 300.0;

const SKY_BLUE: Color = Color::new(0.529, 0.808, 0.922, 1.0);
const BROWN: Color = Color::new(0.545, 0.271, 0.075, 1.0);
const DARK_BROWN: Color = Color::new(0.396, 0.263, 0.129, 1.0);
const FOREST_GREEN: Color = Color::new(0.133, 0.545, 0.133, 1.0);
const GOLD: Color = Color::new(1.0, 0.843, 0.0, 1.0);
const CRIMSON: Color = Color::new(0.863, 0.078, 0.235, 1.0);

// --- 游戏状态 ---
#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Idle,
    Growing,
    Falling,
    Crossing,
    Dead,
    Transitioning,
}

// --- 平台 ---
struct Platform {
    x: f32,
    width: f32,
}

impl Platform {
    fn new(x: f32, width: f32) -> Self {
        Self { x, width }
    }

    fn right_edge(&self) -> f32 {
        self.x + self.width
    }

    fn left_edge(&self) -> f32 {
        self.x
    }

    fn draw(&self, camera_x: f32) {
        let dx = self.x - camera_x;
        let y = SCREEN_HEIGHT - PLATFORM_HEIGHT;

        draw_rectangle(dx, y, self.width, PLATFORM_HEIGHT, BROWN);
        draw_rectangle_lines(dx, y, self.width, PLATFORM_HEIGHT, 3.0, DARK_BROWN);
        draw_rectangle(dx, y, self.width, 10.0, FOREST_GREEN);
    }
}

// --- 平台管理器 ---
struct PlatformManager {
    current: Platform,
    next: Platform,
}

impl PlatformManager {
    fn new() -> Self {
        let current = Platform::new(0.0, 150.0);
        let next = Self::generate_next_from(&current);
        Self { current, next }
    }

    fn generate_next_from(current: &Platform) -> Platform {
        let mut rng = ::rand::thread_rng();
        let gap = rng.gen_range(GAP_MIN..=GAP_MAX) as f32;
        let width = rng.gen_range(PLATFORM_MIN_WIDTH..=PLATFORM_MAX_WIDTH) as f32;
        let x = current.right_edge() + gap;
        Platform::new(x, width)
    }

    fn advance(&mut self) {
        let next = Self::generate_next_from(&self.next);
        self.current = std::mem::replace(&mut self.next, next);
    }

    fn draw(&self, camera_x: f32) {
        self.current.draw(camera_x);
        self.next.draw(camera_x);
    }
}

// --- 金箍棒 ---
struct Staff {
    length: f32,
    angle: f32, // 度数，90=竖直向上，0=水平向右
    base_x: f32,
    base_y: f32,
    growing: bool,
    falling: bool,
    landed: bool,
}

impl Staff {
    fn new() -> Self {
        Self {
            length: 0.0,
            angle: 90.0,
            base_x: 0.0,
            base_y: 0.0,
            growing: false,
            falling: false,
            landed: false,
        }
    }

    fn start_grow(&mut self, x: f32, y: f32) {
        self.length = 0.0;
        self.angle = 90.0;
        self.base_x = x;
        self.base_y = y;
        self.growing = true;
        self.falling = false;
        self.landed = false;
    }

    fn grow(&mut self, dt: f32) {
        if self.growing {
            self.length += STAFF_GROW_SPEED * dt;
        }
    }

    fn start_fall(&mut self) {
        self.growing = false;
        self.falling = true;
    }

    fn fall(&mut self, dt: f32) {
        if self.falling && self.angle > 0.0 {
            self.angle -= STAFF_ROTATE_SPEED * dt;
            if self.angle <= 0.0 {
                self.angle = 0.0;
                self.falling = false;
                self.landed = true;
            }
        }
    }

    fn get_end_x_when_landed(&self) -> f32 {
        self.base_x + self.length
    }

    fn reset(&mut self) {
        self.length = 0.0;
        self.angle = 90.0;
        self.growing = false;
        self.falling = false;
        self.landed = false;
    }

    fn draw(&self, camera_x: f32) {
        if self.length <= 0.0 {
            return;
        }
        let rad = self.angle.to_radians();
        let start_x = self.base_x - camera_x;
        let start_y = self.base_y;
        let end_x = self.base_x + self.length * rad.cos() - camera_x;
        let end_y = self.base_y - self.length * rad.sin();
        draw_line(start_x, start_y, end_x, end_y, STAFF_WIDTH, GOLD);
    }
}

// --- 猴子 ---
struct Monkey {
    x: f32,
    y: f32,
    falling: bool,
    fall_speed: f32,
    dead: bool,
}

impl Monkey {
    fn new(x: f32) -> Self {
        Self {
            x,
            y: SCREEN_HEIGHT - PLATFORM_HEIGHT - MONKEY_SIZE,
            falling: false,
            fall_speed: 0.0,
            dead: false,
        }
    }

    fn walk_to(&mut self, target_x: f32, dt: f32) -> bool {
        if self.x < target_x {
            self.x += MONKEY_SPEED * dt;
            if self.x >= target_x {
                self.x = target_x;
                return true;
            }
        }
        false
    }

    fn start_fall(&mut self) {
        self.falling = true;
        self.fall_speed = 0.0;
    }

    fn update_fall(&mut self, dt: f32) {
        if self.falling {
            self.fall_speed += 800.0 * dt;
            self.y += self.fall_speed * dt;
            if self.y > SCREEN_HEIGHT + 100.0 {
                self.dead = true;
            }
        }
    }

    fn set_position(&mut self, x: f32) {
        self.x = x;
        self.y = SCREEN_HEIGHT - PLATFORM_HEIGHT - MONKEY_SIZE;
    }

    fn draw(&self, camera_x: f32) {
        let dx = self.x - camera_x;
        let dy = self.y;

        // 身体
        draw_ellipse(
            dx + MONKEY_SIZE / 2.0,
            dy + 15.0 + (MONKEY_SIZE - 15.0) / 2.0,
            MONKEY_SIZE / 2.0,
            (MONKEY_SIZE - 15.0) / 2.0,
            0.0,
            ORANGE,
        );

        // 头
        draw_ellipse(dx + 8.0 + 12.0, dy + 11.0, 12.0, 11.0, 0.0, ORANGE);

        // 脸
        let face_color = Color::new(1.0, 0.863, 0.706, 1.0);
        draw_ellipse(dx + 12.0 + 8.0, dy + 6.0 + 7.0, 8.0, 7.0, 0.0, face_color);

        // 眼睛
        draw_circle(dx + 16.0, dy + 11.0, 3.0, WHITE);
        draw_circle(dx + 24.0, dy + 11.0, 3.0, WHITE);
        draw_circle(dx + 16.0, dy + 11.0, 1.0, BLACK);
        draw_circle(dx + 24.0, dy + 11.0, 1.0, BLACK);
    }
}

// --- 游戏主结构 ---
struct Game {
    platforms: PlatformManager,
    monkey: Monkey,
    staff: Staff,
    state: GameState,
    score: u32,
    camera_x: f32,
    target_camera_x: f32,
}

impl Game {
    fn new() -> Self {
        let platforms = PlatformManager::new();
        let monkey_x = platforms.current.right_edge() - MONKEY_SIZE - 10.0;
        let monkey = Monkey::new(monkey_x);
        Self {
            platforms,
            monkey,
            staff: Staff::new(),
            state: GameState::Idle,
            score: 0,
            camera_x: 0.0,
            target_camera_x: 0.0,
        }
    }

    fn reset(&mut self) {
        self.platforms = PlatformManager::new();
        self.monkey = Monkey::new(
            self.platforms.current.right_edge() - MONKEY_SIZE - 10.0,
        );
        self.staff = Staff::new();
        self.state = GameState::Idle;
        self.score = 0;
        self.camera_x = 0.0;
        self.target_camera_x = 0.0;
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.state {
                GameState::Idle => {
                    self.state = GameState::Growing;
                    let staff_x = self.platforms.current.right_edge();
                    let staff_y = SCREEN_HEIGHT - PLATFORM_HEIGHT;
                    self.staff.start_grow(staff_x, staff_y);
                }
                GameState::Dead => {
                    self.reset();
                }
                _ => {}
            }
        }

        if is_mouse_button_released(MouseButton::Left) && self.state == GameState::Growing {
            self.state = GameState::Falling;
            self.staff.start_fall();
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Growing => {
                self.staff.grow(dt);
            }
            GameState::Falling => {
                self.staff.fall(dt);
                if self.staff.landed {
                    self.state = GameState::Crossing;
                }
            }
            GameState::Crossing => {
                let target_x = self.get_cross_target();
                let arrived = self.monkey.walk_to(target_x, dt);
                if arrived {
                    let staff_end = self.staff.get_end_x_when_landed();
                    let next_left = self.platforms.next.left_edge();
                    if staff_end >= next_left {
                        self.on_success();
                    } else {
                        self.monkey.start_fall();
                        self.state = GameState::Dead;
                    }
                }
            }
            GameState::Dead => {
                self.monkey.update_fall(dt);
            }
            GameState::Transitioning => {
                self.camera_x += (self.target_camera_x - self.camera_x) * 0.05;
                if (self.camera_x - self.target_camera_x).abs() < 1.0 {
                    self.camera_x = self.target_camera_x;
                    self.state = GameState::Idle;
                }
            }
            GameState::Idle => {}
        }
    }

    fn get_cross_target(&self) -> f32 {
        let staff_end = self.staff.get_end_x_when_landed();
        let next_left = self.platforms.next.left_edge();
        if staff_end >= next_left {
            self.platforms.next.left_edge() + MONKEY_SIZE
        } else {
            staff_end
        }
    }

    fn on_success(&mut self) {
        self.score += 1;
        self.staff.reset();
        self.platforms.advance();
        self.monkey.set_position(
            self.platforms.current.right_edge() - MONKEY_SIZE - 10.0,
        );
        self.target_camera_x = self.platforms.current.x - 50.0;
        self.state = GameState::Transitioning;
    }

    fn draw(&self) {
        clear_background(SKY_BLUE);

        self.platforms.draw(self.camera_x);
        self.staff.draw(self.camera_x);
        self.monkey.draw(self.camera_x);

        // 分数
        draw_text(
            &format!("Score: {}", self.score),
            10.0,
            30.0,
            36.0,
            BLACK,
        );

        if self.state == GameState::Idle {
            draw_text(
                "Hold mouse to extend staff",
                SCREEN_WIDTH / 2.0 - 160.0,
                60.0,
                28.0,
                BLACK,
            );
        }

        if self.state == GameState::Dead && self.monkey.dead {
            let text = "GAME OVER";
            let font_size = 72.0;
            let text_width = measure_text(text, None, font_size as u16, 1.0).width;
            draw_text(
                text,
                SCREEN_WIDTH / 2.0 - text_width / 2.0,
                SCREEN_HEIGHT / 3.0,
                font_size,
                CRIMSON,
            );
            draw_text(
                "Click to retry",
                SCREEN_WIDTH / 2.0 - 80.0,
                SCREEN_HEIGHT / 3.0 + 50.0,
                28.0,
                BLACK,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "我的眼睛就是尺".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();
        game.handle_input();
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
