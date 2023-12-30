extern crate raylib;

use raylib::prelude::*;
use rand::Rng;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGTH: i32 = 720;

struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
}

struct Paddle {
    position: Vector2,
    size: Vector2,
    speed: f32,
}

struct Pong {
    ball: Ball,
    paddle_left: Paddle,
    paddle_right: Paddle,
    score: (i32, i32),
}

impl Pong {
    fn new() -> Self {
        let ball_radius: f32 = 15.0;
        let ball_velocity: Vector2 = Vector2::new(8.0, 8.0);
        let paddle_size: Vector2 = Vector2::new(20.0, 120.0);
        let paddle_speed: f32 = 8.0;

        Self {
            ball: Ball {
                position: Vector2::new(SCREEN_WIDTH as f32 / 2.0,SCREEN_HEIGTH as f32 / 2.0),
                velocity: ball_velocity,
                radius: ball_radius,
            },
            paddle_left: Paddle {
                position: Vector2::new(5.0, SCREEN_HEIGTH as f32 / 2.0),
                size: paddle_size, 
                speed: paddle_speed,
            },
            paddle_right: Paddle {
                position: Vector2::new(SCREEN_WIDTH as f32 - 25.0, SCREEN_HEIGTH as f32 / 2.0),
                size: paddle_size, 
                speed: paddle_speed,
            },
            score: (0, 0),
        }
    }

    fn update(&mut self, rl: &RaylibHandle){
        // left player keyboard input
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_D) {
            self.paddle_left.position.y -= self.paddle_left.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_A) {
            self.paddle_left.position.y += self.paddle_left.speed;
        }

        // right player keyboard input
        if rl.is_key_down(KeyboardKey::KEY_UP)|| rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.paddle_right.position.y -= self.paddle_right.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.paddle_right.position.y += self.paddle_right.speed;
        }

        clamp(&mut self.paddle_right.position.y, 0.0, SCREEN_HEIGTH as f32 - 100.0);
        clamp(&mut self.paddle_left.position.y, 0.0, SCREEN_HEIGTH as f32 - 100.0);

        // update ball position
        self.ball.position.x += self.ball.velocity.x;
        self.ball.position.y += self.ball.velocity.y;
        
        if self.paddle_collision(&self.paddle_left) || self.paddle_collision(&self.paddle_right) {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        self.wall_collision();
    }

    fn paddle_collision(&self, paddle: &Paddle) -> bool {
        let ball_left = self.ball.position.x - self.ball.radius;
        let ball_right = self.ball.position.x + self.ball.radius;
        let ball_top = self.ball.position.y - self.ball.radius;
        let ball_bottom = self.ball.position.y + self.ball.radius;

        let paddle_left = paddle.position.x;
        let paddle_right = paddle.position.x + paddle.size.x;
        let paddle_top = paddle.position.y;
        let paddle_bottom = paddle.position.y + paddle.size.y;

        ball_left < paddle_right && ball_right > paddle_left && ball_top < paddle_bottom && ball_bottom > paddle_top
    }

    fn wall_collision(&mut self) {
        // check if ball hits top or botton
        if self.ball.position.y as i32 >= SCREEN_HEIGTH || self.ball.position.y <= 0.0 {
            self.ball.velocity.y = -self.ball.velocity.y; 
        }

        if self.ball.position.x as i32 >= SCREEN_WIDTH {
            self.score.0 += 1;
            self.ball.position.x = SCREEN_WIDTH as f32 / 2.0;
            self.ball.velocity.x = -self.ball.velocity.x;
            self.ball.position.y = rand::thread_rng().gen_range(0..=SCREEN_HEIGTH) as f32;
        }
        else if self.ball.position.x as i32 <= 0 {
            self.score.1 += 1;
            self.ball.position.x = SCREEN_WIDTH as f32 / 2.0;
            self.ball.velocity.x = -self.ball.velocity.x;
            self.ball.position.y = rand::thread_rng().gen_range(0..=SCREEN_HEIGTH) as f32;
        }
    }
}

fn clamp(val: &mut f32, min: f32, max: f32) {
    *val = val.max(min).min(max);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGTH)
        .title("Pong")
        .build();

    let entity_color = Color::WHITE;
    let background_color = Color::BLACK;
    let font_color = Color::WHITE;
    let font_size = 80;
    rl.set_target_fps(60);

    let mut pong: Pong = Pong::new();
    while !rl.window_should_close() { 
        pong.update(&rl);

        // start drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background_color);

        d.draw_rectangle(pong.paddle_left.position.x as i32,
            pong.paddle_left.position.y as i32,
            pong.paddle_left.size.x as i32,
            pong.paddle_left.size.y as i32,
            entity_color,
        );

        d.draw_rectangle(pong.paddle_right.position.x as i32,
            pong.paddle_right.position.y as i32,
            pong.paddle_right.size.x as i32,
            pong.paddle_right.size.y as i32,
            entity_color,
        );

        d.draw_circle(pong.ball.position.x as i32,
            pong.ball.position.y as i32,
            pong.ball.radius,
            entity_color,
        );

        let score = format!("{} - {}", pong.score.0, pong.score.1);
        d.draw_text(&score, (SCREEN_WIDTH / 2) - font_size, SCREEN_HEIGTH / 2, font_size, font_color);
        std::mem::drop(score);
    }
}

