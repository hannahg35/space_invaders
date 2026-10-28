use crate::ui::image_button::ImageButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32) -> (String, TextureManager, i32) {
    let title = StillImage::from_preload(
        tm.get_preload("assets/title.png").unwrap(),
        screen_width(),  // Width
        screen_height(), // Height
        0.0,             // X position
        0.0,             // Y position
        true,            // Enable stretching
        1.0,             // Zoom level (100%)
    );
    let scale = (screen_width() / 1500.0).min(screen_height() / 1000.0);
    let button_width = 280.0 * scale;
    let button_height = 140.0 * scale;
    let button_spacing = 120.0 * scale;
    let button_y = screen_height() * 0.8;
    let button_start_x =
        (screen_width() - (button_width * 3.0 + button_spacing * 2.0)) / 2.0;

    let btn_tut = ImageButton::new(
        button_start_x + button_width + button_spacing,
        button_y,
        button_width,
        button_height,
        "assets/tut.png",      // normal state image
        "assets/tutpress.png", // hover state image
    )
    .await;

    let btn_play = ImageButton::new(
        button_start_x,
        button_y,
        button_width,
        button_height,
        "assets/play.png",      // normal state image
        "assets/playpress.png", // hover state image
    )
    .await;

    let btn_exit = ImageButton::new(
        button_start_x + (button_width + button_spacing) * 2.0,
        button_y,
        button_width,
        button_height,
        "assets/exit.png",
        "assets/exitpress.png",
    )
    .await;

    loop {
        clear_background(WHITE);
        title.draw();

        if btn_play.click() {
            return ("game".to_string(), tm, score);
        }

        if btn_tut.click() {
            return ("tut".to_string(), tm, score);
        }

        if btn_exit.click() {
            return ("exit".to_string(), tm, score);
        }

        next_frame().await;
    }
}
