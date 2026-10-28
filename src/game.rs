
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use crate::utils::collision::check_collision;
use crate::custom::player::Player;
use crate::custom::laser::Laser;
use crate::custom::enemy::Enemy;

use macroquad::prelude::*;

fn clone_preloaded_image(
    preloaded: &(Texture2D, Option<Vec<u8>>, String),
) -> (Texture2D, Option<Vec<u8>>, String) {
    (
        preloaded.0.clone(),
        preloaded.1.clone(),
        preloaded.2.clone(),
    )
}

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){

    let player_texture = tm.get_preload("assets/xwing.png").unwrap();
    let enemy_texture = tm.get_preload("assets/tiefighter.png").unwrap();
    let laser_texture = tm.get_preload("assets/laser.png").unwrap();

    let mut player = Player::from_preload(
    clone_preloaded_image(&player_texture),
    screen_width() / 2.0 - screen_width() * 0.045, // X position
    screen_height() * (1.0 - 0.11), // Y position: keep the larger ship at the screen bottom
    screen_width() * 0.09,  // Width
    screen_height() * 0.11, // Height
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    5.0,                    // Speed
    vec2(35.0, 15.0),       // Old position
);

    let background= StillImage::from_preload(
    tm.get_preload("assets/background.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let sides= StillImage::from_preload(
    tm.get_preload("assets/sides.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut lasers: Vec<Laser> = Vec::new();
let mut enemies: Vec<Enemy> = Vec::new();
let columns = 8;
let rows = 3;
let spacing_x = screen_width() * 0.095;
let spacing_y = screen_height() * 0.12;
let enemy_width = screen_width() * 0.07;
let start_x = screen_width() / 2.0
    - spacing_x * (columns - 1) as f32 / 2.0
    - enemy_width / 2.0;
let start_y = screen_height() * 0.1;

for row in 0..rows {
    for col in 0..columns {
        let enemy = Enemy::from_preload(
            clone_preloaded_image(&enemy_texture),
            start_x + col as f32 * spacing_x,
            start_y + row as f32 * spacing_y,
            enemy_width,
            screen_height() * 0.08,
            true,
            1.0,
            3.0,
        );
        enemies.push(enemy);
    }
}

let mut shoot_cooldown = 0.0;
let mut enemy_direction = 1.0_f32;
let enemy_speed = screen_width() * 0.04;
let mut game_result: Option<bool> = None;
let mut result_started_at = None;


    loop {
        clear_background(WHITE);
        background.draw();
        for enemy in &enemies {
            enemy.get_image().draw();
        }
        player.get_image().draw();

        if let Some(won) = game_result {
            sides.draw();
            let message = if won { "YOU WIN!" } else { "YOU LOSE!" };
            let message_color = Color::from_hex(0xFFDE59);
            let font_size = 100.0;
            let text_width = measure_text(message, None, font_size as u16, 1.0).width;
            let box_width = text_width + 100.0;
            let box_height = 180.0;
            let box_x = (screen_width() - box_width) / 2.0;
            let box_y = (screen_height() - box_height) / 2.0;
            draw_rectangle(
                box_x,
                box_y,
                box_width,
                box_height,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(
                message,
                (screen_width() - text_width) / 2.0,
                box_y + 115.0,
                font_size,
                message_color,
            );

            if get_time() - result_started_at.unwrap_or(get_time()) >= 2.5 {
                return ("title".to_string(), tm, score);
            }

            next_frame().await;
            continue;
        }

        player.keypress();
        player.move_player();
        if !enemies.is_empty() {
            let frame_distance = enemy_speed * get_frame_time() * enemy_direction;
            for enemy in &mut enemies {
                enemy.move_formation(frame_distance, 0.0);
            }

            let hit_sides = enemies
                .iter()
                .any(|enemy| check_collision(enemy.get_image(), &sides, 1));

            if hit_sides {
                enemy_direction *= -1.0;
                for enemy in &mut enemies {
                    enemy.move_formation(-frame_distance, screen_height() * 0.02);
                }
            }
        }

        if enemies.iter().any(|enemy| {
            check_collision(enemy.get_image(), player.get_image(), 1)
                || enemy.image.get_y() + enemy.image.size().y >= player.image.get_y()
        }) {
            game_result = Some(false);
            result_started_at = Some(get_time());
            next_frame().await;
            continue;
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &sides, 1) {
                player.move_back_x();
            }
        }

        let time = get_frame_time();
        shoot_cooldown -= time;

         if is_key_down(KeyCode::Space) && shoot_cooldown <= 0.0 {
        shoot_cooldown = 0.75;

        let laser = Laser::from_preload(
            clone_preloaded_image(&laser_texture),
            player.image.get_x() + 25.0,
            player.image.get_y(),
            screen_width() * 0.01,
            screen_height() * 0.05,
            true,
            1.0,
            3.0,
        );

        lasers.push(laser);
    }

    for laser_index in (0..lasers.len()).rev() {
        let laser = &mut lasers[laser_index];
        laser.shoot();
        laser.get_image().draw();

        if laser.image.get_y() < -100.0 {
            laser.image.clear();
            laser.move_dir = vec2(0.0, 0.0);
            lasers.remove(laser_index);
            continue;
        }

        for enemy_index in (0..enemies.len()).rev() {
            if check_collision(laser.get_image(), enemies[enemy_index].get_image(), 1) {
                enemies.remove(enemy_index);
                lasers.remove(laser_index);
                break;
            }
        }
    }

    if enemies.is_empty() {
        game_result = Some(true);
        result_started_at = Some(get_time());
    }

    let mut last_shot = 0.0;

if is_key_down(KeyCode::Space) && get_time() - last_shot > 1.0 {
    last_shot = get_time();
}



    sides.draw();

        next_frame().await;
    }
}