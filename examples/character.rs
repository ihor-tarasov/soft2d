use soft2d::{core::*, image::*, window::*};

const SRC_TILE_SIZE: IVec2 = IVec2::splat(48);
const PLAYER_SIZE: f32 = 0.5;

struct Character {
    image: Image,
}

impl Character {
    fn new() -> Self {
        let image = Image::open("examples/character.png");
        Self { image }
    }
}

impl State for Character {
    fn render(&mut self, window: &mut Window, _dt: f32) {
        let size = window.size();
        let scale = size.y.min(size.x) as f32;

        let player_size = IVec2::splat((scale * PLAYER_SIZE) as i32);
        let player_pos = size / 2 - player_size / 2;

        let mut buffer = window.buffer();
        buffer.clear(Color::LIGHT_GRAY);
        buffer.blit(
            &self.image,
            None,
            Some(SRC_TILE_SIZE),
            Some(player_pos),
            Some(player_size),
        );
        buffer.present();
    }
}

fn main() {
    soft2d::window::run(
        Config {
            title: "Character",
            width: 640,
            height: 480,
        },
        |_| Character::new(),
    );
}
