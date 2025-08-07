use soft2d::{core::*, image::*, window::*};

const SRC_TILE_SIZE: i32 = 48;
const PLAYER_SIZE: f32 = 0.5;
const PLAYER_SPEED: f32 = 1.0;

#[derive(Clone, Copy)]
struct Timer {
    interval: f32,
    elapsed: f32,
}

impl Timer {
    const fn new(interval: f32) -> Self {
        Self {
            interval,
            elapsed: 0.0,
        }
    }

    const fn update(&mut self, dt: f32) -> i32 {
        self.elapsed += dt;
        let mut count = 0;
        while self.elapsed >= self.interval {
            self.elapsed -= self.interval;
            count += 1;
        }
        count
    }
}

#[derive(Clone, Copy)]
struct Animation {
    frame: i32,
    frames_count: i32,
    timer: Timer,
    src_frame_start: IVec2,
}

impl Animation {
    const fn new(frames_count: i32, interval: f32, src_frame_start: IVec2) -> Self {
        Self {
            frame: 0,
            frames_count,
            timer: Timer::new(interval),
            src_frame_start,
        }
    }

    fn update(&mut self, dt: f32) {
        let frames = self.timer.update(dt);
        self.frame = (self.frame + frames) % self.frames_count;
    }

    fn src_pos(&self) -> IVec2 {
        (self.src_frame_start + ivec2(self.frame, 0)) * SRC_TILE_SIZE
    }
}

const ANIMATIONS: [Animation; 8] = [
    Animation::new(5, 0.1, ivec2(0, 0)),
    Animation::new(5, 0.1, ivec2(0, 1)),
    Animation::new(8, 0.1, ivec2(0, 2)),
    Animation::new(8, 0.1, ivec2(0, 3)),
    Animation::new(5, 0.1, ivec2(0, 4)),
    Animation::new(5, 0.1, ivec2(0, 5)),
    Animation::new(8, 0.1, ivec2(0, 6)),
    Animation::new(8, 0.1, ivec2(0, 7)),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AnimationKind {
    Idle,
    Run,
    Shoot,
    Die,
}

impl AnimationKind {
    const fn animation(self, direction: Direction) -> Animation {
        ANIMATIONS[self as usize * 2 + direction as usize]
    }
}

struct Player {
    pos: Vec2,
    image: Image,
    animation: Animation,
    animation_kind: AnimationKind,
    direction: Direction,
    shoot: Option<Timer>,
}

impl Player {
    fn new() -> Self {
        let image = Image::open("examples/character.png");
        let animation_kind = AnimationKind::Idle;
        let direction = Direction::Right;
        Self {
            pos: Vec2::ZERO,
            image,
            animation: animation_kind.animation(direction),
            animation_kind,
            direction,
            shoot: None,
        }
    }

    fn update(&mut self, window: &mut Window, dt: f32) {
        if let Some(shoot) = self.shoot.as_mut() {
            if shoot.update(dt) != 0 {
                self.shoot = None;
            }
        } else {
            let mut delta = Vec2::ZERO;
            let mut direction = self.direction;
            let mut animation_kind = self.animation_kind;
            let mut is_running = false;
            if window.is_key_pressed(KeyCode::KeyW) {
                animation_kind = AnimationKind::Run;
                delta.y -= 1.0;
                is_running = true;
            }
            if window.is_key_pressed(KeyCode::KeyA) {
                animation_kind = AnimationKind::Run;
                direction = Direction::Left;
                delta.x -= 1.0;
                is_running = true;
            }
            if window.is_key_pressed(KeyCode::KeyS) {
                animation_kind = AnimationKind::Run;
                delta.y += 1.0;
                is_running = true;
            }
            if window.is_key_pressed(KeyCode::KeyD) {
                animation_kind = AnimationKind::Run;
                direction = Direction::Right;
                delta.x += 1.0;
                is_running = true;
            }
            if !is_running {
                animation_kind = AnimationKind::Idle;
            }
            if window.is_key_pressed(KeyCode::KeyB) {
                animation_kind = AnimationKind::Shoot;
                self.shoot = Some(Timer::new(0.5));
            }
            if animation_kind != self.animation_kind || direction != self.direction {
                self.animation = animation_kind.animation(direction);
            }
            self.animation_kind = animation_kind;
            self.direction = direction;
            self.pos += delta.normalize_or_zero() * PLAYER_SPEED * dt;
        }

        self.animation.update(dt);
    }

    fn render(&self, buffer: &mut Buffer, scale: f32, camera_offset: IVec2) {
        let player_size = IVec2::splat((scale * PLAYER_SIZE) as i32);
        let camera_offset = camera_offset - player_size / 2;
        let player_pos = camera_offset + (self.pos * scale).as_ivec2();
        buffer.blit(
            &self.image,
            Some(self.animation.src_pos()),
            Some(IVec2::splat(SRC_TILE_SIZE)),
            Some(player_pos),
            Some(player_size),
        );
    }
}

struct Character {
    player: Player,
}

impl Character {
    fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }
}

impl State for Character {
    fn render(&mut self, window: &mut Window, dt: f32) {
        self.player.update(window, dt);

        let size = window.size();
        let scale = size.y.min(size.x) as f32;
        let camera_offset = size / 2;
        let mut buffer = window.buffer();
        buffer.clear(Color::LIGHT_GRAY);
        self.player.render(&mut buffer, scale, camera_offset);
        buffer.present();
    }
}

fn main() {
    soft2d::window::run(
        Config {
            title: "Character",
            width: 640,
            height: 480,
            target_fps: None,
        },
        Character::new(),
    );
}
