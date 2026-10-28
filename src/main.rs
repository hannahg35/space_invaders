/*
By: <Hannah Ursula Greig>
Date: 2026-10-21
Program Details: <A reacreation of Space Invaders, but with a Star Wars Theme.>
*/

mod ui;
mod utils;
mod custom;
mod title;
mod tut;
mod game;


use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions;


use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(),
        window_width: 1500,
        window_height: 1000,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut tm = TextureManager::new();

    let loading_options = LoadingScreenOptions {
        title: Some("SPACE INVADERS".to_string()),
        background_color: BLACK,
        bar_background_color: Color::from_hex(0x252A36),
        bar_fill_color: Color::from_hex(0xFFE81F),
        text_color: Color::from_hex(0xFFE81F),
        filename_color: Color::from_hex(0x4DA6FF),
        completion_message: "READY FOR BATTLE".to_string(),
        ..Default::default()
    };

    tm.preload_with_loading_screen(
        &[
            "assets/back.png",
            "assets/background.png",
            "assets/backpress.png",
            "assets/exit.png",
            "assets/exitpress.png",
            "assets/laser.png",
            "assets/play.png",
            "assets/playpress.png",
            "assets/sides.png",
            "assets/tiefighter.png",
            "assets/title.png",
            "assets/tut.png",
            "assets/tutpress.png",
            "assets/xwing.png",
        ],
        None,
        Some(loading_options),
    )
    .await;

    let mut score = 0;
    
    let mut current_screen = "title".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, score) = match current_screen.as_str() {
                "title" => title::run(tm, score).await,
                "tut" => tut::run(tm, score).await,
                "game" => game::run(tm, score).await,
        
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}