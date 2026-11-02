use crate::ui::image_button::ImageButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32) -> (String, TextureManager, i32) {
    let tutorial_font = load_ttf_font("assets/AstroSpace-eZ2Bg.ttf").await.expect("Failed to load tutorial font");
    let background = StillImage::from_preload(
        tm.get_preload("assets/background.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );
    let back_button = ImageButton::from_preload(
        (screen_width() - 100.0) / 2.0,
        screen_height() * 0.8,
        100.0,
        50.0,
        tm.get_preload("assets/back.png").unwrap(),
        tm.get_preload("assets/backpress.png").unwrap(),
    );

    loop {
        clear_background(BLACK);
        background.draw();

        let panel_width = screen_width() * 0.7;
        let panel_height = screen_height() * 0.55;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::new(0.0, 0.0, 0.0, 0.75));

        let title = "HOW TO PLAY";
        let title_size = 48.0;
        let title_width = measure_text(title, Some(&tutorial_font), title_size as u16, 1.0).width;
        draw_text_ex(
            title,
            (screen_width() - title_width) / 2.0,
            panel_y + 75.0,
            TextParams {
                font: Some(&tutorial_font),
                font_size: title_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );

        let instructions = [
            "Move left and right with A and D or the arrow keys.",
            "Press Space to fire lasers at the TIE fighters.",
            "Avoid letting the enemies reach your ship.",
            "If you want to leave mid game, press the Esc key.",
            "WARNING: THIS WILL DELETE ANY PROGRESS MADE!",
        ];
        for (index, instruction) in instructions.iter().enumerate() {
            let font_size = 28.0;
            let text_width = measure_text(instruction, Some(&tutorial_font), font_size as u16, 1.0).width;
            draw_text_ex(
                instruction,
                (screen_width() - text_width) / 2.0,
                panel_y + 165.0 + index as f32 * 55.0,
                TextParams {
                    font: Some(&tutorial_font),
                    font_size: font_size as u16,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        if back_button.click() || is_key_pressed(KeyCode::Escape) {
            return ("title".to_string(), tm, score);
        }

        next_frame().await;
    }
}
