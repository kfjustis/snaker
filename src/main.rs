use raylib::prelude::*;

enum PlayerDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Default for PlayerDir {
    fn default() -> Self { PlayerDir::RIGHT }
}

#[derive(Default)]
struct Player {
    pub position: Vector2,
    pub size: f32,
    pub speed: f32,
    pub last_dir: PlayerDir,
}

struct Game {
    player: Player,
}

impl Default for Game {
    fn default() -> Game {
        let player = Player::default();

        Game {
            player
        }
    }
}

fn main() {
    let (mut rl, rt) = raylib::init()
        .size(800, 600)
        .title("snaker | kfjustis | v0.1.0")
        .build();
    
    let mut game = Game::default();

    // Set a different exit key.
    rl.set_exit_key(Some(
        raylib::consts::KeyboardKey::KEY_BACKSPACE));

    // Initialize the game object.
    init_game(&mut game, &rl);
    
    while !rl.window_should_close() {  
        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &rt);
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    // Init. the player position.
    game.player.position = Vector2::new(
        rl.get_screen_width() as f32 * 0.5,
        rl.get_screen_height() as f32 * 0.5,
    );
    game.player.size = 20.0;
    game.player.speed = 8.0;
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;

    // Calculate frame independent speed.
    let dt = rl.get_frame_time();
    let speed = game.player.size * game.player.speed * dt;

    // Check the keys.
    if rl.is_key_down(KEY_UP) {
        game.player.last_dir = PlayerDir::UP;
    }
    if rl.is_key_pressed(KEY_DOWN) {
        game.player.last_dir = PlayerDir::DOWN;
    }
    if rl.is_key_pressed(KEY_LEFT) {
        game.player.last_dir = PlayerDir::LEFT;
    }
    if rl.is_key_pressed(KEY_RIGHT) {
        game.player.last_dir = PlayerDir::RIGHT;
    }

    // Update player position based on the key.
    match game.player.last_dir {
        PlayerDir::UP => game.player.position.y -= speed,
        PlayerDir::DOWN => game.player.position.y += speed,
        PlayerDir::LEFT => game.player.position.x -= speed,
        PlayerDir::RIGHT => game.player.position.x += speed,
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, rt: &RaylibThread)
{
    let mut d = rl.begin_drawing(rt);
    d.clear_background(Color::RAYWHITE);
    d.draw_rectangle(
        game.player.position.x as i32,
        game.player.position.y as i32,
        game.player.size as i32,
        game.player.size as i32,
        Color::BLUE,
    );
}