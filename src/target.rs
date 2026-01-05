use std::{fs::File, io::BufWriter, path::Path};

use image::{Rgb, RgbImage};

pub struct Target {
    width: u32,
    height: u32,
}

impl Target {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn write_png(&self, path: impl AsRef<Path>) {
        let mut image = RgbImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                image.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }

        let mut writer = BufWriter::new(File::create(path).expect("Failed to create file"));

        image
            .write_to(&mut writer, image::ImageFormat::Png)
            .expect("Failed to write image to file");
    }
}
