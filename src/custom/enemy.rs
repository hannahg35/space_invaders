use crate::ui::still_image::StillImage;
use macroquad::prelude::*;

pub struct Enemy {
    pub image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
}
impl Enemy {
    pub fn from_preload(
        preloaded: (Texture2D, Option<Vec<u8>>, String),
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        zoom: f32,
        speed: f32,
    ) -> Self {
        let image = StillImage::from_preload(preloaded, width, height, x, y, stretch, zoom);

        Enemy {
            image,
            speed,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
        speed: f32,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Enemy {
            image,
            speed,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn move_formation(&mut self, horizontal_distance: f32, drop_distance: f32) {
        self.image.set_x(self.image.get_x() + horizontal_distance);
        self.image.set_y(self.image.get_y() + drop_distance);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
   
}
