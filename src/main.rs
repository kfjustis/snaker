use raylib::prelude::*;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
enum PlayerDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Default for PlayerDir {
    fn default() -> Self {
        PlayerDir::RIGHT
    }
}

#[derive(Default, Copy, Clone)]
struct Player {
    pub position: Vector2,
    pub size: f32,
}

struct Food {
    pub position: Vector2,
    pub size: f32,
    pub color: Color,
}

struct Game {
    player_parts: VecDeque<Player>,
    last_dir : PlayerDir ,
    food : Food,
}

impl Default for Game {
    fn default() -> Game {
        let player = Player::default();
        let mut player_parts : VecDeque<Player> = VecDeque::new();
        player_parts.push_back(player);

        let last_dir = PlayerDir::RIGHT;

        let food = Food {
            position: Vector2{x:0.0, y:0.0},
            size: 20.0,
            color: Color::YELLOW,
        };
        
        Game {
            player_parts,
            last_dir,
            food,
        }
    }
}

fn main() {
    let (mut rl, rt) = raylib::init()
        .size(800, 600)
        .title("snaker | kfjustis | v0.2.0")
        .build();
    
    let mut game = Game::default();

    // Set the target frame rate.
    rl.set_target_fps(3);

    // Set a different exit key.
    rl.set_exit_key(Some(
        raylib::consts::KeyboardKey::KEY_BACKSPACE));

    // Initialize the game object.
    init_game(&mut game, &rl);
    
    // Run the game loop.
    while !rl.window_should_close() {  
        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &rt);
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    // Init. the food position.
    game.food.position = Vector2::new(
        rl.get_screen_width() as f32 * 0.333,
        rl.get_screen_height() as f32 * 0.333,
    );

    // Init. the player position and size.
    game.player_parts[0].position = Vector2::new(
        rl.get_screen_width() as f32 * 0.5,
        rl.get_screen_height() as f32 * 0.5,
    );
    game.player_parts[0].size = 20.0;
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;

    // Check the keys.
    if rl.is_key_down(KEY_UP) {
        game.last_dir = PlayerDir::UP;
    }
    if rl.is_key_pressed(KEY_DOWN) {
        game.last_dir = PlayerDir::DOWN;
    }
    if rl.is_key_pressed(KEY_LEFT) {
        game.last_dir = PlayerDir::LEFT;
    }
    if rl.is_key_pressed(KEY_RIGHT) {
        game.last_dir = PlayerDir::RIGHT;
    }

    // Update player position based on the key.
    if game.player_parts.len() == 1 {
        let mut head = &mut game.player_parts[0];
        println!("moved player {:?}", head.position);
        match game.last_dir {
            PlayerDir::UP => head.position.y -= head.size,
            PlayerDir::DOWN => head.position.y += head.size,
            PlayerDir::LEFT => head.position.x -= head.size,
            PlayerDir::RIGHT => head.position.x += head.size,
        }
    }
    else
    {
        let mut tail = game.player_parts.pop_back().unwrap();
        let head = game.player_parts.front().unwrap();

        match game.last_dir {
            PlayerDir::UP => {
                tail.position.x = head.position.x;
                tail.position.y = head.position.y - head.size
            },
            PlayerDir::DOWN => {
                tail.position.x = head.position.x;
                tail.position.y = head.position.y + head.size;
            },
            PlayerDir::LEFT => {
                tail.position.x = head.position.x - head.size;
                tail.position.y = head.position.y;
            },
            PlayerDir::RIGHT => {
                tail.position.x = head.position.x + head.size;
                tail.position.y = head.position.y;
            },
        }

        game.player_parts.push_front(tail);
    }

    // Check food collision.
    if (game.player_parts[0].position.x > game.food.position.x - game.food.size &&
        game.player_parts[0].position.x < game.food.position.x + game.food.size) &&
       (game.player_parts[0].position.y > game.food.position.y - game.food.size &&
        game.player_parts[0].position.y < game.food.position.y + game.food.size) {
            // Get random x and y, then set it to new food position.
            let rand_x = get_random_value(0, rl.get_screen_width());
            let rand_y = get_random_value(0, rl.get_screen_height());
            game.food.position = Vector2::new (rand_x as f32, rand_y as f32);

            // Add a new part to the head of the snake.
            let head = game.player_parts.front();
            let mut new_part = Player {
                position : head.unwrap().position,
                size : head.unwrap().size,
            };
            match game.last_dir {
                PlayerDir::UP => new_part.position.y -= head.unwrap().size,
                PlayerDir::DOWN => new_part.position.y += head.unwrap().size,
                PlayerDir::LEFT => new_part.position.x -= head.unwrap().size,
                PlayerDir::RIGHT => new_part.position.x += head.unwrap().size,
            }
            game.player_parts.push_back(new_part);
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, rt: &RaylibThread)
{
    let mut d = rl.begin_drawing(rt);
    d.clear_background(Color::RAYWHITE);

    // Draw the food.
    d.draw_rectangle(
        game.food.position.x as i32,
        game.food.position.y as i32,
        game.food.size as i32,
        game.food.size as i32,
        game.food.color,
    );

    // Draw the snake.
    for part in game.player_parts.iter() {
        match part {
            _ => {
                d.draw_rectangle(
                    part.position.x as i32,
                    part.position.y as i32,
                    part.size as i32,
                    part.size as i32,
                    Color::BLUE,
                );
            }
        }
    }
}